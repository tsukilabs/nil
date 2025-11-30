// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::infrastructure::building::academy::AcademyRecruitCatalog;
use nil_payload::infrastructure::academy::{
  AddAcademyRecruitOrderRequest,
  CancelAcademyRecruitOrderRequest,
  GetAcademyRecruitCatalogRequest,
};

impl Client {
  /// POST `/infrastructure/academy/recruit/add`
  pub async fn add_academy_recruit_order(&self, req: AddAcademyRecruitOrderRequest) -> Result<()> {
    self
      .http
      .post("infrastructure/academy/recruit/add", req)
      .await
  }

  /// POST `/infrastructure/academy/recruit/cancel`
  pub async fn cancel_academy_recruit_order(
    &self,
    req: CancelAcademyRecruitOrderRequest,
  ) -> Result<()> {
    self
      .http
      .post("infrastructure/academy/recruit/cancel", req)
      .await
  }

  /// POST `/infrastructure/academy/recruit/catalog`
  pub async fn get_academy_recruit_catalog(
    &self,
    req: GetAcademyRecruitCatalogRequest,
  ) -> Result<AcademyRecruitCatalog> {
    self
      .http
      .post_json("infrastructure/academy/recruit/catalog", req)
      .await
  }
}
