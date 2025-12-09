// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use std::{env, io};
use tracing::subscriber::set_global_default;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

const TIMESTAMP: &str = "%F %T%.3f";

pub fn setup() -> Result<()> {
  let mut filter = EnvFilter::builder()
    .from_env()?
    .add_directive("nil=trace".parse()?)
    .add_directive("nil_client=trace".parse()?)
    .add_directive("nil_core=trace".parse()?)
    .add_directive("nil_server=trace".parse()?);

  if env::var("NIL_LOG_TOWER_HTTP").is_ok_and(|it| it == "true") {
    filter = filter.add_directive("tower_http=trace".parse()?);
  }

  let subscriber = Registry::default().with(
    Layer::default()
      .with_ansi(true)
      .with_timer(ChronoLocal::new(TIMESTAMP.into()))
      .with_writer(io::stderr)
      .pretty(),
  );

  set_global_default(subscriber.with(filter))?;

  Ok(())
}
