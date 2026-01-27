// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::resources::{Food, Iron, Resources, Stone, Wood};
use nil_core::ruler::Ruler;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatGetResourcesRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetFoodRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub food: Food,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetIronRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub iron: Iron,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxFoodRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxIronRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxResourcesRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxSiloResourcesRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxStoneRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxWarehouseResourcesRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxWoodRequest {
  pub world: WorldId,
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetResourcesRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub resources: Resources,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetStoneRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub stone: Stone,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetWoodRequest {
  pub world: WorldId,
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub wood: Wood,
}
