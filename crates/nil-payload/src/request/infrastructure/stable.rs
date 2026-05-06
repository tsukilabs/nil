// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::infrastructure::building::stable::recruit_queue::{
  StableRecruitOrderId,
  StableRecruitOrderRequest,
};
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AddStableRecruitOrderRequest {
  pub world: WorldId,
  pub request: StableRecruitOrderRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CancelStableRecruitOrderRequest {
  pub world: WorldId,
  pub coord: Coord,
  pub id: StableRecruitOrderId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetStableRecruitCatalogRequest {
  pub world: WorldId,
  pub coord: Coord,
}
