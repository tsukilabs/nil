// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod prefecture;
pub mod stable;

use nil_core::continent::Coord;
use nil_core::infrastructure::building::BuildingId;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleBuildingRequest {
  pub world: WorldId,
  pub coord: Coord,
  pub id: BuildingId,
  pub enabled: bool,
}
