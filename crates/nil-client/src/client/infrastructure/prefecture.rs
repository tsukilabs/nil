// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::infrastructure::building::prefecture::PrefectureBuildCatalog;
use nil_payload::infrastructure::prefecture::{
  AddPrefectureBuildOrderRequest,
  CancelPrefectureBuildOrderRequest,
  GetPrefectureBuildCatalogRequest,
};

impl Client {
  /// POST `/infrastructure/prefecture/build/add`
  pub async fn add_prefecture_build_order(
    &self,
    req: AddPrefectureBuildOrderRequest,
  ) -> Result<()> {
    self
      .http
      .post("infrastructure/prefecture/build/add", req)
      .await
  }

  /// POST `/infrastructure/prefecture/build/cancel`
  pub async fn cancel_prefecture_build_order(
    &self,
    req: CancelPrefectureBuildOrderRequest,
  ) -> Result<()> {
    self
      .http
      .post("infrastructure/prefecture/build/cancel", req)
      .await
  }

  /// POST `/infrastructure/prefecture/build/catalog`
  pub async fn get_prefecture_build_catalog(
    &self,
    req: GetPrefectureBuildCatalogRequest,
  ) -> Result<PrefectureBuildCatalog> {
    self
      .http
      .post_json("infrastructure/prefecture/build/catalog", req)
      .await
  }
}
