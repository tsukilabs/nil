// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use nil_log::{Directives, Layers};
use nil_server::remote::start_remote;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
  let layers = if cfg!(debug_assertions) { Layers::STDERR } else { Layers::all() };
  let _guard = nil_log::setup(&nil_log::Options {
    directives: Directives::all(),
    layers,
  })?;

  let database_url = env::var("NIL_DATABASE_URL")?;
  start_remote(&database_url).await?;

  Ok(())
}
