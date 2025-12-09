// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::InfrastructureStats;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldStats {
  pub(super) infrastructure: Arc<InfrastructureStats>,
}

#[expect(clippy::new_without_default)]
impl WorldStats {
  pub fn new() -> Self {
    Self {
      infrastructure: Arc::new(InfrastructureStats::new()),
    }
  }

  #[inline]
  pub fn infrastructure(&self) -> Arc<InfrastructureStats> {
    Arc::clone(&self.infrastructure)
  }
}
