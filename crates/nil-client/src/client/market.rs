// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::market::*;
use nil_payload::response::market::*;

impl Client {
  /// Endpoint: `PUT /get-market-fee`
  pub async fn get_market_fee(&self, req: GetMarketFeeRequest) -> Result<GetMarketFeeResponse> {
    http::json_put("get-market-fee")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
