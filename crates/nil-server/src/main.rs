// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use nil_log::{Directives, Layers};
use nil_server::remote;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
  let Some(_guard) = nil_log::setup()
    .directives(Directives::all())
    .debug_layers(Layers::STDERR)
    .release_layers(Layers::FILE)
    .call()?
  else {
    unreachable!();
  };

  let database_url = env::var("NIL_DATABASE_URL")?;
  remote::start(&database_url).await?;

  Ok(())
}
