// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use clap::Parser;
use nil_log::{Directives, Layers};
use nil_server::remote;
use std::env;

#[derive(Debug, Parser)]
#[command(name = "nil-server")]
pub struct Cli {
  #[arg(short = 'u', long)]
  database_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
  let Some(guard) = nil_log::setup()
    .directives(Directives::all())
    .layers(Layers::FILE)
    .call()?
  else {
    unreachable!();
  };

  let cli = Cli::parse();
  let database_url = cli
    .database_url
    .or_else(|| env::var("NIL_DATABASE_URL").ok())
    .expect("Missing database url");

  remote::start(&database_url).await?;

  drop(guard);

  Ok(())
}
