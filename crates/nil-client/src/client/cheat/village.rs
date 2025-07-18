// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::village::Stability;

impl Client {
  /// POST `/cheat/village/stability`
  pub async fn cheat_set_stability(&self, coord: Coord, stability: Stability) -> Result<()> {
    self
      .http
      .post("cheat/village/stability", (coord, stability))
      .await
  }
}
