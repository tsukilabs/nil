// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::infrastructure::building::stable::StableRecruitCatalog;
use nil_payload::infrastructure::stable::{
  AddStableRecruitOrderRequest,
  CancelStableRecruitOrderRequest,
  GetStableRecruitCatalogRequest,
};

impl Client {
  /// POST `/infrastructure/stable/recruit/add`
  pub async fn add_stable_recruit_order(&self, req: AddStableRecruitOrderRequest) -> Result<()> {
    self
      .http
      .post("infrastructure/stable/recruit/add", req)
      .await
  }

  /// POST `/infrastructure/stable/recruit/cancel`
  pub async fn cancel_stable_recruit_order(
    &self,
    req: CancelStableRecruitOrderRequest,
  ) -> Result<()> {
    self
      .http
      .post("infrastructure/stable/recruit/cancel", req)
      .await
  }

  /// POST `/infrastructure/stable/recruit/catalog`
  pub async fn get_stable_recruit_catalog(
    &self,
    req: GetStableRecruitCatalogRequest,
  ) -> Result<StableRecruitCatalog> {
    self
      .http
      .post_json("infrastructure/stable/recruit/catalog", req)
      .await
  }
}
