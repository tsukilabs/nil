// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::continent::Coord;
use nil_core::npc::bot::PublicBot;
use nil_payload::npc::bot::*;

impl Client {
  pub async fn get_bot_coords(&self, req: GetBotCoordsRequest) -> Result<Vec<Coord>> {
    http::json_put("get-bot-coords")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_bot(&self, req: GetPublicBotRequest) -> Result<PublicBot> {
    http::json_put("get-public-bot")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_bots(&self, req: GetPublicBotsRequest) -> Result<Vec<PublicBot>> {
    http::json_put("get-public-bots")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
