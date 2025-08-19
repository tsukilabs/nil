// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::city::{City, PublicCity};
use nil_core::continent::Coord;
use nil_core::ranking::Score;

impl Client {
  /// POST `/city`
  pub async fn get_city(&self, coord: Coord) -> Result<City> {
    self.http.post_json("city", coord).await
  }

  /// POST `/city/public`
  pub async fn get_public_city(&self, coord: Coord) -> Result<PublicCity> {
    self
      .http
      .post_json("city/public", coord)
      .await
  }

  /// POST `/city/rename`
  pub async fn rename_city(&self, coord: Coord, name: &str) -> Result<()> {
    self
      .http
      .post("city/rename", (coord, name))
      .await
  }

  /// POST `/city/score`
  pub async fn get_city_score(&self, coord: Coord) -> Result<Score> {
    self
      .http
      .post_json("city/score", coord)
      .await
  }
}
