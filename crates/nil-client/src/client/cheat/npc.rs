// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::ethic::Ethics;
use nil_core::npc::bot::BotId;
use nil_payload::cheat::npc::*;

impl Client {
  pub async fn cheat_get_ethics(&self, req: CheatGetEthicsRequest) -> Result<Option<Ethics>> {
    http::json_post("cheat-get-ethics")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_set_bot_ethics(&self, req: CheatSetBotEthicsRequest) -> Result<()> {
    http::post("cheat-set-bot-ethics")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_spawn_bot(&self, req: CheatSpawnBotRequest) -> Result<BotId> {
    http::json_post("cheat-spawn-bot")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
