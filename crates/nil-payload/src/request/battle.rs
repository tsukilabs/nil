// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::battle::luck::Luck;
use nil_core::infrastructure::building::BuildingLevel;
use nil_core::military::squad::Squad;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct SimulateBattleRequest {
  pub world: WorldId,
  #[serde(default)]
  pub attacker: Vec<Squad>,
  #[serde(default)]
  pub defender: Vec<Squad>,
  #[serde(default)]
  pub luck: Luck,
  #[serde(default)]
  pub wall: BuildingLevel,
}
