// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::resources::{Food, Iron, Resources, Stone, Wood};
use nil_core::ruler::Ruler;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatGetResourcesRequest {
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetFoodRequest {
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub food: Food,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetIronRequest {
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub iron: Iron,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxFoodRequest {
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxIronRequest {
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxResourcesRequest {
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxSiloResourcesRequest {
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxStoneRequest {
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxWarehouseResourcesRequest {
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CheatSetMaxWoodRequest {
  pub ruler: Option<Ruler>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetResourcesRequest {
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub resources: Resources,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetStoneRequest {
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub stone: Stone,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheatSetWoodRequest {
  #[serde(default)]
  pub ruler: Option<Ruler>,
  pub wood: Wood,
}
