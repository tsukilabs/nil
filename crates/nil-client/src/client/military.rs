// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::military::*;
use nil_payload::response::military::*;

impl Client {
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
