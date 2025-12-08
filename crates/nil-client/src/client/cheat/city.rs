// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_payload::cheat::city::CheatSetStabilityRequest;

impl Client {
  pub async fn cheat_set_stability(&self, req: CheatSetStabilityRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-stability", req)
      .await
  }
}
