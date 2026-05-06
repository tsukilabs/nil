// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod prefecture;
pub mod stable;
pub mod workshop;

use nil_core::continent::Coord;
use nil_core::infrastructure::building::BuildingId;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ToggleBuildingRequest {
  pub world: WorldId,
  pub coord: Coord,
  pub id: BuildingId,
  pub enabled: bool,
}
