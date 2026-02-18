// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::battle::BattleResult;
use nil_payload::battle::*;

impl Client {
  pub async fn simulate_battle(&self, req: SimulateBattleRequest) -> Result<BattleResult> {
    http::json_post("simulate-battle")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
