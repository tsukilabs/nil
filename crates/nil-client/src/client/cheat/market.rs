// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::cheat::market::*;

impl Client {
  /// Endpoint: `POST /cheat-set-market-fee`
  pub async fn cheat_set_market_fee(&self, req: CheatSetMarketFeeRequest) -> Result<()> {
    http::post("cheat-set-market-fee")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
