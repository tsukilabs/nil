// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::{Result, anyhow};
use clap::Parser;
use futures::future::BoxFuture;
use nil_client::{Client, ServerAddr};
use nil_core::event::Event;
use nil_core::player::PlayerId;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use nil_lua::Lua;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Parser)]
#[command(version)]
struct Cli {
  script: PathBuf,

  #[arg(long)]
  player: Option<String>,

  #[arg(long)]
  player_password: Option<String>,

  #[arg(long)]
  world: Option<String>,

  #[arg(long)]
  world_password: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();
  let chunk = fs::read_to_string(cli.script)?;

  let player = cli
    .player
    .or_else(|| env::var("NIL_PLAYER").ok())
    .map(PlayerId::from);

  let player_password = cli
    .player_password
    .or_else(|| env::var("NIL_PLAYER_PASSWORD").ok())
    .as_deref()
    .map(Password::new);

  let world = cli
    .world
    .or_else(|| env::var("NIL_WORLD").ok())
    .as_deref()
    .map(WorldId::try_from)
    .transpose()
    .map_err(|err| anyhow!("Invalid world id").context(err))?;

  let world_password = cli
    .world_password
    .or_else(|| env::var("NIL_WORLD_PASSWORD").ok())
    .as_deref()
    .map(Password::new);

  let mut client = Client::new_remote();
  client
    .update(ServerAddr::Remote)
    .maybe_player_id(player)
    .maybe_player_password(player_password)
    .maybe_world_id(world)
    .maybe_world_password(world_password)
    .maybe_on_event(None::<fn(Event) -> BoxFuture<'static, ()>>)
    .call()
    .await?;

  let mut lua = Lua::with_client(client)?;
  let output = lua.execute(&chunk).await?;

  println!("{}", output.stdout);

  Ok(())
}
