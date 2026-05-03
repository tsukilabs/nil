// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::infrastructure::building::prefecture::build_queue::PrefectureBuildOrderRequest;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPrefectureBuildOrderRequest {
  pub world: WorldId,
  pub request: PrefectureBuildOrderRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelPrefectureBuildOrderRequest {
  pub world: WorldId,
  pub coord: Coord,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPrefectureBuildCatalogRequest {
  pub world: WorldId,
  pub coord: Coord,
}
