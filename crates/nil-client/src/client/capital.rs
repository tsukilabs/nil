// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::capital::*;
use nil_payload::response::capital::*;

impl Client {
  /// Endpoint: `PUT /get-city-limit`
  pub async fn get_city_limit(&self, req: GetCityLimitRequest) -> Result<GetCityLimitResponse> {
    http::json_put("get-city-limit")
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
