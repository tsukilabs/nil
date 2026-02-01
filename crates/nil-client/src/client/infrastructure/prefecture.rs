// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::infrastructure::building::prefecture::PrefectureBuildCatalog;
use nil_payload::infrastructure::prefecture::*;

impl Client {
  pub async fn add_prefecture_build_order(
    &self,
    req: AddPrefectureBuildOrderRequest,
  ) -> Result<()> {
    http::post("add-prefecture-build-order")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cancel_prefecture_build_order(
    &self,
    req: CancelPrefectureBuildOrderRequest,
  ) -> Result<()> {
    http::post("cancel-prefecture-build-order")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_prefecture_build_catalog(
    &self,
    req: GetPrefectureBuildCatalogRequest,
  ) -> Result<PrefectureBuildCatalog> {
    http::json_post("get-prefecture-build-catalog")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
