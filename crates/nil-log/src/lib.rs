// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use bitflags::bitflags;
use std::{env, io};
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer as _, Registry};

const TIMESTAMP: &str = "%F %T%.3f";

pub fn setup(options: &Options) -> Result<LogGuard> {
  let mut guard = LogGuard::default();
  let mut filter = EnvFilter::builder().from_env()?;

  macro_rules! add_directive {
    ($flag:ident, $directive:literal) => {
      if options
        .directives
        .contains(Directives::$flag)
      {
        filter = filter.add_directive($directive.parse()?);
      }
    };
    ($flag:ident, $directive:literal, $env_var:literal) => {
      if options
        .directives
        .contains(Directives::$flag)
        || env::var($env_var).is_ok_and(|it| it == "true")
      {
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

  let mut layers = Vec::new();

  // The order matters.
  // See: https://github.com/tokio-rs/tracing/issues/1817
  if options.layers.contains(Layers::FILE)
    && let Ok(path) = env::var("NIL_LOG_DIR")
  {
    let appender = RollingFileAppender::builder()
      .rotation(Rotation::DAILY)
      .filename_suffix("nil.log")
      .max_log_files(30)
      .build(path)?;

    let (non_blocking, worker_guard) = tracing_appender::non_blocking(appender);
    layers.push(
      Layer::default()
        .with_ansi(false)
        .with_timer(ChronoLocal::new(TIMESTAMP.into()))
        .with_writer(non_blocking)
        .pretty()
        .boxed(),
    );

    guard.worker_guard = Some(worker_guard);
  }

  if options.layers.contains(Layers::STDERR) {
    layers.push(
      Layer::default()
        .with_ansi(true)
        .with_timer(ChronoLocal::new(TIMESTAMP.into()))
        .with_writer(io::stderr)
        .pretty()
        .boxed(),
    );
  }

  let subscriber = Registry::default().with(layers);
  set_global_default(subscriber.with(filter))?;

  Ok(guard)
}

#[inline]
pub fn setup_default() -> Result<LogGuard> {
  setup(&Options::default())
}

#[must_use]
#[derive(Default)]
pub struct LogGuard {
  worker_guard: Option<WorkerGuard>,
}

#[derive(Clone, Debug, Default)]
pub struct Options {
  pub directives: Directives,
  pub layers: Layers,
}

impl Options {
  pub fn with_directives(directives: Directives) -> Self {
    Self { directives, ..Default::default() }
  }

  pub fn with_layers(layers: Layers) -> Self {
    Self { layers, ..Default::default() }
  }
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

impl Directives {
  pub fn nil() -> Directives {
    Self::NIL | Self::NIL_CLIENT | Self::NIL_CORE | Self::NIL_DATABASE | Self::NIL_SERVER
  }
}

impl Default for Directives {
  fn default() -> Self {
    Self::nil()
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
    Self::all()
  }
}
