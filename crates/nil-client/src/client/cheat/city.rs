// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_payload::cheat::city::*;

impl Client {
  pub async fn cheat_set_stability(&self, req: CheatSetStabilityRequest) -> Result<()> {
    http::post("cheat-set-stability")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_deref())
      .send()
      .await
  }
}
