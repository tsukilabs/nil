// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::ethic::Ethics;
use nil_core::npc::bot::BotId;
use nil_core::ruler::Ruler;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatGetEthicsRequest {
  pub ruler: Ruler,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetBotEthicsRequest {
  pub id: BotId,
  pub ethics: Ethics,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSpawnBotRequest {
  pub name: String,
}
