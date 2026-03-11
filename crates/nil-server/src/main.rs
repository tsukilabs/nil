// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use mimalloc::MiMalloc;
use nil_log::{Directives, Layers};
use nil_server::remote;
use std::env;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
  let _guard = nil_log::setup()
    .directives(Directives::all())
    .debug_layers(Layers::STDERR)
    .release_layers(Layers::FILE)
    .call()?;

  let database_url = env::var("NIL_DATABASE_URL")?;
  remote::start(&database_url).await?;

  Ok(())
}
