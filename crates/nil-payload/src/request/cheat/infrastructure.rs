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
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetAcademyRecruitQueuesRequest {
  #[builder(start_fn, into)]
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
  #[builder(start_fn, into)]
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
  #[builder(start_fn, into)]
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
  #[builder(start_fn, into)]
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
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetPrefectureBuildQueueRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetPrefectureBuildQueuesRequest {
  #[builder(start_fn, into)]
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
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CheatGetStableRecruitQueuesRequest {
  #[builder(start_fn, into)]
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
  #[builder(start_fn, into)]
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
  #[builder(start_fn, into)]
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
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}
