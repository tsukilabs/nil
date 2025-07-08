// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::PublicField;
use nil_core::village::Coord;

impl Client {
  /// POST `/continent/field`
  pub async fn get_field(&self, coord: Coord) -> Result<PublicField> {
    self
      .http
      .post_json("continent/field", coord)
      .await
  }

  /// POST `/continent/fields`
  pub async fn get_fields(&self, coords: Vec<Coord>) -> Result<Vec<(Coord, PublicField)>> {
    self
      .http
      .post_json("continent/fields", coords)
      .await
  }

  /// GET `/continent/size`
  pub async fn get_continent_size(&self) -> Result<usize> {
    self.http.get_json("continent/size").await
  }
}
