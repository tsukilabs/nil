// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::infrastructure::building::workshop::recruit_queue::{
  WorkshopRecruitOrderId,
  WorkshopRecruitOrderRequest,
};
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddWorkshopRecruitOrderRequest {
  pub world: WorldId,
  pub request: WorkshopRecruitOrderRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelWorkshopRecruitOrderRequest {
  pub world: WorldId,
  pub coord: Coord,
  pub id: WorkshopRecruitOrderId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWorkshopRecruitCatalogRequest {
  pub world: WorldId,
  pub coord: Coord,
}
