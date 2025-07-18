// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod prefecture;

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::infrastructure::building::BuildingId;

impl Client {
  /// POST `/infrastructure/toggle`
  pub async fn toggle_building(&self, coord: Coord, id: BuildingId, enabled: bool) -> Result<()> {
    self
      .http
      .post("infrastructure/toggle", (coord, id, enabled))
      .await
  }
}
