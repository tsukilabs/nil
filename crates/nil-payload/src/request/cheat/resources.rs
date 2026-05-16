// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::resources::{Food, Iron, Resources, Stone, Wood};
use nil_core::ruler::Ruler;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatGetResourcesRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetFoodRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  #[builder(default, into)]
  pub food: Food,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetIronRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  #[builder(default, into)]
  pub iron: Iron,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxFoodRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxIronRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxResourcesRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxSiloResourcesRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxStoneRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxWarehouseResourcesRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxWoodRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetResourcesRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  #[builder(default)]
  pub resources: Resources,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetStoneRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  #[builder(default, into)]
  pub stone: Stone,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetWoodRequest {
  #[builder(start_fn)]
  pub world: WorldId,
  #[serde(default)]
  #[builder(into)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  #[builder(default, into)]
  pub wood: Wood,
}
