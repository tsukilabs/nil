// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::continent::coord::Coord;
use nil_core::infrastructure::building::r#impl::academy::recruit_queue::{
  AcademyRecruitOrderId,
  AcademyRecruitOrderRequest,
};
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct AddAcademyRecruitOrderRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub request: AcademyRecruitOrderRequest,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct CancelAcademyRecruitOrderRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
  pub id: AcademyRecruitOrderId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetAcademyRecruitCatalogRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub coord: Coord,
}
