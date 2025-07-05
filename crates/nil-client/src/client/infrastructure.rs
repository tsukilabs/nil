// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::infrastructure::building::prefecture::{
  PrefectureBuildOrderOptions,
  PrefectureCatalog,
};
use nil_core::village::Coord;

impl Client {
  /// POST `/infrastructure/prefecture/build`
  pub async fn add_prefecture_build_order(
    &self,
    options: PrefectureBuildOrderOptions,
  ) -> Result<()> {
    self
      .http
      .post("infrastructure/prefecture/build", options)
      .await
  }

  /// POST `/infrastructure/prefecture/cancel-build`
  pub async fn cancel_prefecture_build_order(&self, coord: Coord) -> Result<()> {
    self
      .http
      .post("infrastructure/prefecture/cancel-build", coord)
      .await
  }

  /// POST `/infrastructure/prefecture/catalog`
  pub async fn get_prefecture_catalog(&self, coord: Coord) -> Result<PrefectureCatalog> {
    self
      .http
      .post_json("infrastructure/prefecture/catalog", coord)
      .await
  }
}
