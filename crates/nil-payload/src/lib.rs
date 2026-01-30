// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod battle;
pub mod chat;
pub mod cheat;
pub mod city;
pub mod continent;
pub mod infrastructure;
pub mod military;
pub mod npc;
pub mod player;
pub mod ranking;
pub mod report;
pub mod round;
pub mod user;
pub mod world;

use nil_core::player::PlayerId;
use nil_core::world::WorldId;
use nil_server_types::Token;
use nil_util::password::Password;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
  pub player: PlayerId,
  #[serde(default)]
  pub password: Option<Password>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTokenRequest {
  pub token: Token,
}

impl<T> From<T> for ValidateTokenRequest
where
  T: AsRef<str>,
{
  fn from(token: T) -> Self {
    Self { token: Token::new(token) }
  }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsocketQuery {
  pub world_id: WorldId,
  #[serde(default)]
  pub world_password: Option<Password>,
}
