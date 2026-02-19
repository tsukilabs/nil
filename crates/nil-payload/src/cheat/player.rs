// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::player::PlayerId;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatGetPlayerRequest {
  pub world: WorldId,
  #[serde(default)]
  pub player: Option<PlayerId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatGetPlayersRequest {
  pub world: WorldId,
}
