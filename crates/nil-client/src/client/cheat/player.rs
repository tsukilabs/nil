// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::player::Player;
use nil_payload::cheat::player::*;

impl Client {
  pub async fn cheat_get_player(&self, req: CheatGetPlayerRequest) -> Result<Player> {
    http::json_put("cheat-get-player")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_players(&self, req: CheatGetPlayersRequest) -> Result<Vec<Player>> {
    http::json_put("cheat-get-players")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
