// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::ranking::{Ranking, RankingEntry};
use nil_payload::ranking::*;

impl Client {
  pub async fn get_rank(&self, req: GetRankRequest) -> Result<Option<RankingEntry>> {
    http::json_post("get-rank")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn get_ranking(&self, req: GetRankingRequest) -> Result<Ranking> {
    http::json_post("get-ranking")
      .body(req)
      .server(self.server)
      .send()
      .await
  }
}
