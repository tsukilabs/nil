// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::npc::bot::BotId;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBotCoordsRequest {
  pub world: WorldId,
  pub id: BotId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicBotRequest {
  pub world: WorldId,
  pub id: BotId,
}
