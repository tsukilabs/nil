// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::resources::{Food, Iron, Resources, Stone, Wood};
use nil_core::ruler::Ruler;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatGetResourcesRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetFoodRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  pub food: Food,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetIronRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  pub iron: Iron,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxFoodRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxIronRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxResourcesRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxSiloResourcesRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxStoneRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxWarehouseResourcesRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetMaxWoodRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetResourcesRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  pub resources: Resources,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetStoneRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  pub stone: Stone,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct CheatSetWoodRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  #[serde(default)]
  pub wood: Wood,
}
