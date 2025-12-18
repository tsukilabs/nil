// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::ranking::{Ranking, RankingEntry};
use nil_payload::ranking::*;

impl Client {
  pub async fn get_rank(&self, req: GetRankRequest) -> Result<Option<RankingEntry>> {
    self.http.json_post("get-rank", req).await
  }

  pub async fn get_ranking(&self, req: GetRankingRequest) -> Result<Ranking> {
    self.http.json_post("get-ranking", req).await
  }
}
