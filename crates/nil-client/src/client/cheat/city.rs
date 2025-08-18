// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::city::Stability;
use nil_core::continent::Coord;

impl Client {
  /// POST `/cheat/city/stability`
  pub async fn cheat_set_stability(&self, coord: Coord, stability: Stability) -> Result<()> {
    self
      .http
      .post("cheat/city/stability", (coord, stability))
      .await
  }
}
