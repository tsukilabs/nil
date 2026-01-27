// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::infrastructure::building::stable::{StableRecruitOrderId, StableRecruitOrderRequest};
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddStableRecruitOrderRequest {
  pub world: WorldId,
  pub request: StableRecruitOrderRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelStableRecruitOrderRequest {
  pub world: WorldId,
  pub coord: Coord,
  pub id: StableRecruitOrderId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStableRecruitCatalogRequest {
  pub world: WorldId,
  pub coord: Coord,
}
