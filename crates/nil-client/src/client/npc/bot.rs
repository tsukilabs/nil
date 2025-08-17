// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::npc::bot::{BotId, PublicBot};

impl Client {
  /// GET `/npc/bot/{id}/coord`
  pub async fn get_bot_coords(&self, id: BotId) -> Result<Vec<Coord>> {
    self
      .http
      .get_json(&format!("npc/bot/{id}/coord"))
      .await
  }

  /// GET `/npc/bot/{id}/public`
  pub async fn get_public_bot(&self, id: BotId) -> Result<PublicBot> {
    self
      .http
      .get_json(&format!("npc/bot/{id}/public"))
      .await
  }
}
