// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::player::PlayerId;
use nil_core::round::RoundId;
use nil_core::world::{WorldConfig, WorldId};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRemoteWorldRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRemoteWorldResponse {
  pub config: WorldConfig,
  pub created_by: PlayerId,
  pub has_password: bool,
  pub active_players: usize,
  pub current_round: RoundId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWorldConfigRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWorldStatsRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveLocalWorldRequest {
  pub world: WorldId,
  pub path: PathBuf,
}
