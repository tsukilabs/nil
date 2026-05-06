// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::city::Stability;
use nil_core::continent::Coord;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CheatSetStabilityRequest {
  pub world: WorldId,
  pub coord: Coord,
  pub stability: Stability,
}
