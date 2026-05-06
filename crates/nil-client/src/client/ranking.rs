// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::ranking::*;
use nil_payload::response::ranking::*;

impl Client {
  pub async fn get_rank(&self, req: GetRankRequest) -> Result<GetRankResponse> {
    http::json_put("get-rank")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_ranking(&self, req: GetRankingRequest) -> Result<GetRankingResponse> {
    http::json_put("get-ranking")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
