// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::infrastructure::building::academy::{
  AcademyRecruitOrderId,
  AcademyRecruitOrderRequest,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAcademyRecruitOrderRequest {
  pub request: AcademyRecruitOrderRequest,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAcademyRecruitOrderRequest {
  pub coord: Coord,
  pub id: AcademyRecruitOrderId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAcademyRecruitCatalogRequest {
  pub coord: Coord,
}
