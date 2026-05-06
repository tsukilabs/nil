// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::stats::InfrastructureStats;
use crate::world::config::WorldConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorldStats {
  pub(super) infrastructure: Arc<InfrastructureStats>,
}

impl WorldStats {
  pub fn new(config: &WorldConfig) -> Self {
    Self {
      infrastructure: Arc::new(InfrastructureStats::new(config)),
    }
  }

  #[inline]
  pub fn infrastructure(&self) -> Arc<InfrastructureStats> {
    Arc::clone(&self.infrastructure)
  }
}
