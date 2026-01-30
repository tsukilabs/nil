// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use bitflags::bitflags;
use std::{env, fs, io};
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer as _, Registry};

const TIMESTAMP: &str = "%F %T%.3f";

#[bon::builder]
pub fn setup(
  #[builder(default)] directives: Directives,
  #[builder(default)] layers: Layers,
) -> Result<Option<WorkerGuard>> {
  let mut guard = None::<WorkerGuard>;
  let mut filter = EnvFilter::builder().from_env()?;

  macro_rules! add_directive {
    ($flag:ident, $directive:literal) => {
      if directives.contains(Directives::$flag) {
        filter = filter.add_directive($directive.parse()?);
      }
    };
    ($flag:ident, $directive:literal, $env_var:literal) => {
      if directives.contains(Directives::$flag) || env::var($env_var).is_ok_and(|it| it == "true") {
        filter = filter.add_directive($directive.parse()?);
      }
    };
  }

  add_directive!(NIL, "nil=trace");
  add_directive!(NIL_CLIENT, "nil_client=trace");
  add_directive!(NIL_CORE, "nil_core=trace");
  add_directive!(NIL_DATABASE, "nil_database=trace");
  add_directive!(NIL_SERVER, "nil_server=trace");

  add_directive!(TOWER_HTTP, "tower_http=trace", "NIL_LOG_TOWER_HTTP");

  let mut chosen_layers = Vec::new();

  // The order matters.
  // See: https://github.com/tokio-rs/tracing/issues/1817
  if layers.contains(Layers::FILE)
    && let Ok(path) = env::var("NIL_LOG_DIR")
  {
    fs::create_dir_all(&path)?;
    let appender = RollingFileAppender::builder()
      .rotation(Rotation::HOURLY)
      .filename_suffix("nil.log")
      .max_log_files(30 * 24)
      .build(path)?;

    let (writer, worker_guard) = tracing_appender::non_blocking(appender);
    chosen_layers.push(
      Layer::default()
        .with_ansi(false)
        .with_timer(ChronoLocal::new(TIMESTAMP.into()))
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(writer)
        .pretty()
        .boxed(),
    );

    guard = Some(worker_guard);
  }

  if layers.contains(Layers::STDERR) {
    chosen_layers.push(
      Layer::default()
        .with_ansi(true)
        .with_timer(ChronoLocal::new(TIMESTAMP.into()))
        .with_writer(io::stderr)
        .pretty()
        .boxed(),
    );
  }

  let subscriber = Registry::default()
    .with(chosen_layers)
    .with(filter);

  set_global_default(subscriber)?;

  Ok(guard)
}

bitflags! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct Directives: u32 {
    const NIL            = 1 << 0;
    const NIL_CLIENT     = 1 << 1;
    const NIL_CORE       = 1 << 2;
    const NIL_DATABASE   = 1 << 3;
    const NIL_SERVER     = 1 << 4;
    const TOWER_HTTP     = 1 << 5;
  }
}

impl Default for Directives {
  fn default() -> Self {
    Self::all().difference(Self::TOWER_HTTP)
  }
}

bitflags! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct Layers: u32 {
    const FILE       = 1 << 0;
    const STDERR     = 1 << 1;
  }
}

impl Default for Layers {
  fn default() -> Self {
    Self::STDERR
  }
}
