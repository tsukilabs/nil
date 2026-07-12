// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::cheat::city::*;
use nil_payload::response::cheat::city::*;

impl Client {
  pub async fn cheat_get_cities(
    &self,
    req: CheatGetCitiesRequest,
  ) -> Result<CheatGetCitiesResponse> {
    http::json_put("cheat-get-cities")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_get_city(&self, req: CheatGetCityRequest) -> Result<CheatGetCityResponse> {
    http::json_put("cheat-get-city")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_set_stability(&self, req: CheatSetStabilityRequest) -> Result<()> {
    http::post("cheat-set-stability")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn cheat_spawn_city(&self, req: CheatSpawnCityRequest) -> Result<()> {
    http::post("cheat-spawn-city")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
