// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::infrastructure::building::stable::StableRecruitCatalog;
use nil_payload::infrastructure::stable::*;

impl Client {
  pub async fn add_stable_recruit_order(&self, req: AddStableRecruitOrderRequest) -> Result<()> {
    self
      .http
      .post("add-stable-recruit-order", req)
      .await
  }

  pub async fn cancel_stable_recruit_order(
    &self,
    req: CancelStableRecruitOrderRequest,
  ) -> Result<()> {
    self
      .http
      .post("cancel-stable-recruit-order", req)
      .await
  }

  pub async fn get_stable_recruit_catalog(
    &self,
    req: GetStableRecruitCatalogRequest,
  ) -> Result<StableRecruitCatalog> {
    self
      .http
      .json_post("get-stable-recruit-catalog", req)
      .await
  }
}
