// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::ethic::Ethics;
use nil_core::npc::bot::BotId;
use nil_payload::cheat::npc::{CheatGetEthicsRequest, CheatSpawnBotRequest};

impl Client {
  pub async fn cheat_get_ethics(&self, req: CheatGetEthicsRequest) -> Result<Option<Ethics>> {
    self
      .http
      .json_post("cheat-get-ethics", req)
      .await
  }

  pub async fn cheat_spawn_bot(&self, req: CheatSpawnBotRequest) -> Result<BotId> {
    self
      .http
      .json_post("cheat-spawn-bot", req)
      .await
  }
}
