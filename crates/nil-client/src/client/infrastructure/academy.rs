// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::infrastructure::building::academy::AcademyRecruitCatalog;
use nil_payload::infrastructure::academy::*;

impl Client {
  pub async fn add_academy_recruit_order(&self, req: AddAcademyRecruitOrderRequest) -> Result<()> {
    self
      .http
      .post("add-academy-recruit-order", req)
      .await
  }

  pub async fn cancel_academy_recruit_order(
    &self,
    req: CancelAcademyRecruitOrderRequest,
  ) -> Result<()> {
    self
      .http
      .post("cancel-academy-recruit-order", req)
      .await
  }

  pub async fn get_academy_recruit_catalog(
    &self,
    req: GetAcademyRecruitCatalogRequest,
  ) -> Result<AcademyRecruitCatalog> {
    self
      .http
      .json_post("get-academy-recruit-catalog", req)
      .await
  }
}
