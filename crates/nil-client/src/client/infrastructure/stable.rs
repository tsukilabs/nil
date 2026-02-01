// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::infrastructure::building::stable::StableRecruitCatalog;
use nil_payload::infrastructure::stable::*;

impl Client {
  pub async fn add_stable_recruit_order(&self, req: AddStableRecruitOrderRequest) -> Result<()> {
    http::post("add-stable-recruit-order")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cancel_stable_recruit_order(
    &self,
    req: CancelStableRecruitOrderRequest,
  ) -> Result<()> {
    http::post("cancel-stable-recruit-order")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_stable_recruit_catalog(
    &self,
    req: GetStableRecruitCatalogRequest,
  ) -> Result<StableRecruitCatalog> {
    http::json_post("get-stable-recruit-catalog")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
