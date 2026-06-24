// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(const_default, const_trait_impl)]

pub mod timer;

use anyhow::Result;
use bitflags::bitflags;
use std::{fs, io};
use timer::Timer;
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer as _, Registry};

#[bon::builder]
pub fn setup(
  #[builder(default)] directives: Directives,
  #[builder(default)] debug_layers: Layers,
  #[builder(default)] release_layers: Layers,
) -> Result<Option<WorkerGuard>> {
  setup_(directives, debug_layers, release_layers)
}

fn setup_(
  directives: Directives,
  debug_layers: Layers,
  release_layers: Layers,
) -> Result<Option<WorkerGuard>> {
  let mut guard = None::<WorkerGuard>;
  let mut filter = EnvFilter::builder().from_env()?;

  let level = nil_env::log_level();
  let layers = if cfg!(debug_assertions) { debug_layers } else { release_layers };

  macro_rules! add_directive {
    ($flag:ident, $krate:literal) => {
      if directives.contains(Directives::$flag) {
        let directive = format!("{}={}", $krate, level);
        filter = filter.add_directive(directive.parse()?);
      }
    };
    ($flag:ident, $krate:literal, $var_fn:ident) => {
      if directives.contains(Directives::$flag) || nil_env::$var_fn() {
        let directive = format!("{}={}", $krate, level);
        filter = filter.add_directive(directive.parse()?);
      }
    };
  }

  add_directive!(NIL, "nil");
  add_directive!(NIL_CLIENT, "nil_client");
  add_directive!(NIL_CORE, "nil_core");
  add_directive!(NIL_CRYPTO, "nil_crypto");
  add_directive!(NIL_SERVER, "nil_server");
  add_directive!(NIL_SERVER_DATABASE, "nil_server_database");

  add_directive!(TOWER_HTTP, "tower_http", log_tower_http);

  let mut chosen_layers = Vec::new();

  // The order matters.
  // See: https://github.com/tokio-rs/tracing/issues/1817
  if layers.contains(Layers::FILE) {
    let path = nil_env::log_dir()?;
    fs::create_dir_all(&path)?;

    let appender = RollingFileAppender::builder()
      .rotation(Rotation::HOURLY)
      .filename_suffix("log")
      .max_log_files(30 * 24)
      .build(path)?;

    let (writer, worker_guard) = tracing_appender::non_blocking(appender);
    chosen_layers.push(
      Layer::default()
        .with_ansi(false)
        .with_timer(Timer)
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
        .with_timer(Timer)
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
    const NIL                   = 1 << 0;
    const NIL_CLIENT            = 1 << 1;
    const NIL_CORE              = 1 << 2;
    const NIL_CRYPTO            = 1 << 3;
    const NIL_SERVER            = 1 << 4;
    const NIL_SERVER_DATABASE   = 1 << 5;
    const TOWER_HTTP            = 1 << 6;
  }
}

const impl Default for Directives {
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

const impl Default for Layers {
  fn default() -> Self {
    Self::STDERR
  }
}
