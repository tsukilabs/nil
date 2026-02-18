// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(if_let_guard)]

mod config;

use anyhow::{Result, anyhow};
use clap::Parser;
use config::Config;
use futures::future::BoxFuture;
use nil_client::{Client, ServerAddr};
use nil_core::event::Event;
use nil_core::player::PlayerId;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use nil_lua::Lua;
use nil_server_types::Token;
use std::fs;
use std::path::PathBuf;

const USER_AGENT: &str = concat!("nil-lua/", env!("CARGO_PKG_VERSION"));

type OnEvent = fn(Event) -> BoxFuture<'static, ()>;

#[derive(Parser)]
#[command(version)]
struct Cli {
  script: PathBuf,

  #[arg(long)]
  server: Option<ServerAddr>,

  #[arg(long)]
  player: Option<PlayerId>,

  #[arg(long)]
  player_password: Option<Password>,

  #[arg(long)]
  world: Option<String>,

  #[arg(long)]
  world_password: Option<Password>,

  #[arg(long)]
  token: Option<Token>,
}

#[tokio::main]
async fn main() -> Result<()> {
  #[cfg(debug_assertions)]
  nil_log::setup().call()?;

  let cli = Cli::parse();
  let config = Config::load()?;

  let world = cli
    .world
    .as_deref()
    .map(WorldId::try_from)
    .transpose()
    .map_err(|err| anyhow!("Invalid world id").context(err))?
    .or(config.world);

  let mut client = Client::new_remote();
  client.set_user_agent(USER_AGENT);

  macro_rules! or {
    ($key:ident) => {{ cli.$key.or(config.$key) }};
  }

  client
    .update(cli.server.unwrap_or(config.server))
    .maybe_player_id(or!(player))
    .maybe_player_password(or!(player_password))
    .maybe_world_id(world)
    .maybe_world_password(or!(world_password))
    .maybe_authorization_token(or!(token))
    .maybe_on_event(None::<OnEvent>)
    .call()
    .await?;

  let mut lua = Lua::with_client(client)?;
  let chunk = fs::read_to_string(cli.script)?;
  let output = lua.execute(&chunk).await?;

  print!("{}", output.stdout);

  Ok(())
}
