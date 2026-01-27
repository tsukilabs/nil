// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use nil_server::start_remote;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
  let database_url = env::var("NIL_DATABASE_URL")?;
  start_remote(&database_url).await?;

  Ok(())
}
