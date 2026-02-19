// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::player::Player;
use nil_payload::cheat::player::*;

impl Client {
  pub async fn cheat_get_players(&self, req: CheatGetPlayersRequest) -> Result<Vec<Player>> {
    http::json_post("cheat-get-players")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
