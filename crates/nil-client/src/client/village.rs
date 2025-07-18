// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::village::Village;

impl Client {
  /// POST `/village`
  pub async fn get_village(&self, coord: Coord) -> Result<Village> {
    self.http.post_json("village", coord).await
  }

  /// POST `/village/rename`
  pub async fn rename_village(&self, coord: Coord, name: &str) -> Result<()> {
    self
      .http
      .post("village/rename", (coord, name))
      .await
  }
}
