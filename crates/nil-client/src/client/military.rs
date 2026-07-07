// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::military::*;
use nil_payload::response::military::*;

impl Client {
  pub async fn get_army(&self, req: GetArmyRequest) -> Result<GetArmyResponse> {
    http::json_put("get-army")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_army_owner(&self, req: GetArmyOwnerRequest) -> Result<GetArmyOwnerResponse> {
    http::json_put("get-army-owner")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_maneuver(&self, req: GetManeuverRequest) -> Result<GetManeuverResponse> {
    http::json_put("get-maneuver")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn request_maneuver(
    &self,
    req: RequestManeuverRequest,
  ) -> Result<RequestManeuverResponse> {
    http::json_post("request-maneuver")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
