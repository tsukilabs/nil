// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::infrastructure::building::academy::recruit_catalog::AcademyRecruitCatalog;
use nil_payload::infrastructure::academy::*;

impl Client {
  pub async fn add_academy_recruit_order(&self, req: AddAcademyRecruitOrderRequest) -> Result<()> {
    http::post("add-academy-recruit-order")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cancel_academy_recruit_order(
    &self,
    req: CancelAcademyRecruitOrderRequest,
  ) -> Result<()> {
    http::post("cancel-academy-recruit-order")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_academy_recruit_catalog(
    &self,
    req: GetAcademyRecruitCatalogRequest,
  ) -> Result<AcademyRecruitCatalog> {
    http::json_post("get-academy-recruit-catalog")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
