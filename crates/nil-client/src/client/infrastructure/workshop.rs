// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::infrastructure::building::workshop::recruit_catalog::WorkshopRecruitCatalog;
use nil_payload::infrastructure::workshop::*;

impl Client {
  pub async fn add_workshop_recruit_order(
    &self,
    req: AddWorkshopRecruitOrderRequest,
  ) -> Result<()> {
    http::post("add-workshop-recruit-order")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cancel_workshop_recruit_order(
    &self,
    req: CancelWorkshopRecruitOrderRequest,
  ) -> Result<()> {
    http::post("cancel-workshop-recruit-order")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_workshop_recruit_catalog(
    &self,
    req: GetWorkshopRecruitCatalogRequest,
  ) -> Result<WorkshopRecruitCatalog> {
    http::json_post("get-workshop-recruit-catalog")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
