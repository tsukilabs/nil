// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::ranking::{Ranking, RankingEntry, RankingEntryRuler};

impl Client {
  /// GET `/ranking`
  pub async fn get_ranking(&self) -> Result<Ranking> {
    self.http.get_json("ranking").await
  }

  /// POST `/ranking`
  pub async fn get_rank(&self, id: RankingEntryRuler) -> Result<Option<RankingEntry>> {
    self.http.post_json("ranking", id).await
  }
}
