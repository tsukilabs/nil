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
pub mod world;

use nil_core::player::PlayerId;
use nil_core::world::WorldId;
use nil_server_types::Password;
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
pub struct LeaveRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsocketQuery {
  pub world: WorldId,
}
