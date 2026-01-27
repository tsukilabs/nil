// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::battle::Luck;
use nil_core::infrastructure::building::BuildingLevel;
use nil_core::military::squad::Squad;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateBattleRequest {
  pub world: WorldId,
  pub attacker: Vec<Squad>,
  pub defender: Vec<Squad>,
  pub luck: Luck,
  pub wall: BuildingLevel,
}
