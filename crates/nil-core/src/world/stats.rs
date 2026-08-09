// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::stats::InfrastructureStats;
use crate::market::MarketPriceTable;
use crate::world::config::WorldConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct WorldStats {
  pub(super) infrastructure: Arc<InfrastructureStats>,
  market_price_table: MarketPriceTable,
}

impl WorldStats {
  pub fn new(config: &WorldConfig) -> Self {
    Self {
      infrastructure: Arc::new(InfrastructureStats::new(config)),
      market_price_table: MarketPriceTable::default(),
    }
  }

  #[inline]
  pub fn infrastructure(&self) -> Arc<InfrastructureStats> {
    Arc::clone(&self.infrastructure)
  }

  #[inline]
  pub fn market_price_table(&self) -> MarketPriceTable {
    self.market_price_table
  }
}
