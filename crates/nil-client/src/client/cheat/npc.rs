// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::npc::bot::BotId;
use nil_core::npc::precursor::PrecursorId;
use nil_core::resources::Resources;
use nil_payload::cheat::npc::CheatSpawnBotRequest;

impl Client {
  /// POST `/cheat/bot/spawn`
  pub async fn cheat_spawn_bot(&self, req: CheatSpawnBotRequest) -> Result<BotId> {
    self
      .http
      .post_json("cheat/bot/spawn", req)
      .await
  }

  /// GET `/cheat/bot/{id}/resources`
  pub async fn cheat_get_bot_resources(&self, id: BotId) -> Result<Resources> {
    let route = format!("cheat/bot/{id}/resources");
    self.http.get_json(&route).await
  }

  /// GET `/cheat/precursor/{id}/resources`
  pub async fn cheat_get_precursor_resources(&self, id: PrecursorId) -> Result<Resources> {
    let route = format!("cheat/precursor/{id}/resources");
    self.http.get_json(&route).await
  }
}
