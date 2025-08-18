// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::city::{City, PublicCity};
use nil_core::continent::Coord;

impl Client {
  /// POST `/city`
  pub async fn get_city(&self, coord: Coord) -> Result<City> {
    self.http.post_json("city", coord).await
  }

  /// GET `/city/public`
  pub async fn get_public_cities(&self) -> Result<Vec<PublicCity>> {
    self.http.get_json("city/public").await
  }

  /// POST `/city/public`
  pub async fn get_public_city(&self, coord: Coord) -> Result<PublicCity> {
    self
      .http
      .post_json("city/public", coord)
      .await
  }

  /// POST `/city/public-by`
  pub async fn get_public_cities_by(&self, coords: Vec<Coord>) -> Result<Vec<PublicCity>> {
    self
      .http
      .post_json("city/public-by", coords)
      .await
  }

  /// POST `/city/rename`
  pub async fn rename_city(&self, coord: Coord, name: &str) -> Result<()> {
    self
      .http
      .post("city/rename", (coord, name))
      .await
  }
}
