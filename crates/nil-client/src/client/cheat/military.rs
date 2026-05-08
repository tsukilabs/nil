// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::cheat::military::*;
use nil_payload::response::cheat::military::*;

impl Client {
  pub async fn cheat_get_idle_armies_at(
    &self,
    req: CheatGetIdleArmiesAtRequest,
  ) -> Result<CheatGetIdleArmiesAtResponse> {
    http::json_put("cheat-get-idle-armies-at")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_idle_personnel_at(
    &self,
    req: CheatGetIdlePersonnelAtRequest,
  ) -> Result<CheatGetIdlePersonnelAtResponse> {
    http::json_put("cheat-get-idle-personnel-at")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_spawn_personnel(&self, req: CheatSpawnPersonnelRequest) -> Result<()> {
    http::post("cheat-spawn-personnel")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
