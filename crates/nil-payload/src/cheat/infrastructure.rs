// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::infrastructure::prelude::{BuildingId, BuildingLevel};
use nil_core::ruler::Ruler;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatGetAcademyRecruitQueueRequest {
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatGetInfrastructureRequest {
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatGetPrefectureBuildQueueRequest {
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatGetStableRecruitQueueRequest {
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatGetStorageCapacityRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetBuildingLevelRequest {
  pub world: WorldId,
  pub coord: Coord,
  pub id: BuildingId,
  pub level: BuildingLevel,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetMaxInfrastructureRequest {
  pub world: WorldId,
  pub coord: Coord,
}
