// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatGetBuildStepsRequest {
  pub world: WorldId,
  pub coord: Coord,
}
