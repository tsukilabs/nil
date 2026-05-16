// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::continent::Coord;
use nil_core::infrastructure::prelude::{BuildingId, BuildingLevel};
use nil_core::ruler::Ruler;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAcademyRecruitQueueRequest {
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAcademyRecruitQueuesRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(default, with = FromIterator::from_iter)]
  pub coords: Vec<Coord>,
  #[serde(default)]
  #[builder(default)]
  pub filter_empty: bool,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAllAcademyRecruitQueuesRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(default)]
  pub filter_empty: bool,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAllPrefectureBuildQueuesRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(default)]
  pub filter_empty: bool,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAllStableRecruitQueuesRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(default)]
  pub filter_empty: bool,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetInfrastructureRequest {
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetPrefectureBuildQueueRequest {
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetPrefectureBuildQueuesRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(default, with = FromIterator::from_iter)]
  pub coords: Vec<Coord>,
  #[serde(default)]
  #[builder(default)]
  pub filter_empty: bool,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetStableRecruitQueueRequest {
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetStableRecruitQueuesRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(default, with = FromIterator::from_iter)]
  pub coords: Vec<Coord>,
  #[serde(default)]
  #[builder(default)]
  pub filter_empty: bool,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatGetStorageCapacityRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatSetBuildingLevelRequest {
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
  #[builder(into)]
  pub id: BuildingId,
  #[builder(into)]
  pub level: BuildingLevel,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatSetMaxInfrastructureRequest {
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}
