// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::cheat::capital::*;
use nil_payload::response::cheat::capital::*;

impl Client {
  /// Endpoint: `PUT /cheat-get-influence`
  pub async fn cheat_get_influence(
    &self,
    req: CheatGetInfluenceRequest,
  ) -> Result<CheatGetInfluenceResponse> {
    http::json_put("cheat-get-influence")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  /// Endpoint: `POST /cheat-set-influence`
  pub async fn cheat_set_influence(&self, req: CheatSetInfluenceRequest) -> Result<()> {
    http::post("cheat-set-influence")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
