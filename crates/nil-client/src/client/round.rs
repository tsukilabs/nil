// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::round::*;
use nil_payload::response::round::*;

impl Client {
  /// Endpoint: `PUT /get-round`
  pub async fn get_round(&self, req: GetRoundRequest) -> Result<GetRoundResponse> {
    http::json_put("get-round")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  /// Endpoint: `POST /set-player-ready`
  pub async fn set_player_ready(
    &self,
    req: SetPlayerReadyRequest,
  ) -> Result<SetPlayerReadyResponse> {
    http::json_post("set-player-ready")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  /// Endpoint: `POST /start-round`
  pub async fn start_round(&self, req: StartRoundRequest) -> Result<StartRoundResponse> {
    http::json_post("start-round")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
