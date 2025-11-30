// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use std::io;
use tracing::subscriber::set_global_default;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

const TIMESTAMP: &str = "%F %T%.3f";

pub fn setup() {
  let result: Result<()> = try {
    let subscriber = Registry::default()
      .with(
        Layer::default()
          .with_ansi(true)
          .with_timer(ChronoLocal::new(TIMESTAMP.into()))
          .with_writer(io::stderr)
          .pretty(),
      )
      .with(
        EnvFilter::builder()
          .from_env()?
          .add_directive("nil=trace".parse()?)
          .add_directive("nil_client=trace".parse()?)
          .add_directive("nil_core=trace".parse()?)
          .add_directive("nil_server=trace".parse()?)
          .add_directive("tower_http=trace".parse()?),
      );

    set_global_default(subscriber)?;
  };

  result.unwrap();
}
