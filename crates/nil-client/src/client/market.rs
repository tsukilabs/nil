// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::market::*;
use nil_payload::response::market::*;

impl Client {
  /// Endpoint: `POST /buy-resources`
  pub async fn buy_resources(&self, req: BuyResourcesRequest) -> Result<()> {
    http::post("buy-resources")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  /// Endpoint: `PUT /get-market`
  pub async fn get_market(&self, req: GetMarketRequest) -> Result<GetMarketResponse> {
    http::json_put("get-market")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

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

  /// Endpoint: `POST /sell-resources`
  pub async fn sell_resources(&self, req: SellResourcesRequest) -> Result<()> {
    http::post("sell-resources")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  /// Endpoint: `POST /send-resources`
  pub async fn send_resources(&self, req: SendResourcesRequest) -> Result<()> {
    http::post("send-resources")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
