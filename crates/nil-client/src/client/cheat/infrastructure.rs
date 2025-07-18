// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::infrastructure::building::{BuildingId, BuildingLevel};

impl Client {
  /// POST `/cheat/infrastructure`
  pub async fn cheat_set_max_infrastructure(&self, coord: Coord) -> Result<()> {
    self
      .http
      .post("cheat/infrastructure", coord)
      .await
  }

  /// POST `/cheat/infrastructure/building`
  pub async fn cheat_set_building_level(
    &self,
    coord: Coord,
    id: BuildingId,
    level: BuildingLevel,
  ) -> Result<()> {
    self
      .http
      .post("cheat/infrastructure/building", (coord, id, level))
      .await
  }
}
