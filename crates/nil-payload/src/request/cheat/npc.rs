// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::ethic::Ethics;
use nil_core::infrastructure::Infrastructure;
use nil_core::npc::bot::BotId;
use nil_core::ruler::Ruler;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetEthicsRequest {
  pub world: WorldId,
  pub ruler: Ruler,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetBotEthicsRequest {
  pub world: WorldId,
  pub id: BotId,
  #[serde(default)]
  pub ethics: Option<Ethics>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSpawnBotRequest {
  pub world: WorldId,
  pub name: String,
  #[serde(default)]
  pub infrastructure: Option<Infrastructure>,
}
