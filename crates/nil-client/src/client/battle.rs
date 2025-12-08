// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::battle::BattleResult;
use nil_payload::battle::SimulateBattleRequest;

impl Client {
  pub async fn simulate_battle(&self, req: SimulateBattleRequest) -> Result<BattleResult> {
    self
      .http
      .json_post("simulate-battle", req)
      .await
  }
}
