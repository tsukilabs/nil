// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::military::army::ArmyPersonnel;
use nil_core::ruler::Ruler;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSpawnPersonnelRequest {
  pub coord: Coord,
  pub ruler: Option<Ruler>,
  pub personnel: ArmyPersonnel,
}
