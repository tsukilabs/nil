// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::infrastructure::building::prefecture::{
  PrefectureBuildCatalog,
  PrefectureBuildOrderRequest,
};

impl Client {
  /// POST `/infrastructure/prefecture/build/add`
  pub async fn add_prefecture_build_order(&self, req: PrefectureBuildOrderRequest) -> Result<()> {
    self
      .http
      .post("infrastructure/prefecture/build/add", req)
      .await
  }

  /// POST `/infrastructure/prefecture/build/cancel`
  pub async fn cancel_prefecture_build_order(&self, coord: Coord) -> Result<()> {
    self
      .http
      .post("infrastructure/prefecture/build/cancel", coord)
      .await
  }

  /// POST `/infrastructure/prefecture/build/catalog`
  pub async fn get_prefecture_build_catalog(&self, coord: Coord) -> Result<PrefectureBuildCatalog> {
    self
      .http
      .post_json("infrastructure/prefecture/build/catalog", coord)
      .await
  }
}
