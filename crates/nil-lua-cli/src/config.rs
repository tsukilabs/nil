// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use nil_client::ServerAddr;
use nil_core::player::PlayerId;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use nil_server_types::Token;
use serde::Deserialize;
use std::io::ErrorKind;
use std::{env, fs};

const PATH: &str = ".tsukilabs/nil/lua.toml";

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
  pub server: ServerAddr,
  pub player: Option<PlayerId>,
  pub player_password: Option<Password>,
  pub world: Option<WorldId>,
  pub world_password: Option<Password>,
  pub token: Option<Token>,
}

impl Config {
  pub fn load() -> Result<Self> {
    let Some(home) = env::home_dir() else {
      return Ok(Self::default());
    };

    match fs::read(home.join(PATH)) {
      Ok(bytes) => Ok(toml::from_slice(&bytes)?),
      Err(err) if let ErrorKind::NotFound = err.kind() => Ok(Self::default()),
      Err(err) => Err(err.into()),
    }
  }
}
