// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_payload::cheat::city::CheatSetStabilityRequest;

impl Client {
  /// POST `/cheat/city/stability`
  pub async fn cheat_set_stability(&self, req: CheatSetStabilityRequest) -> Result<()> {
    self
      .http
      .post("cheat/city/stability", req)
      .await
  }
}
