// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::resources::Resources;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct MarketVault {
  resources: Resources,
}

impl MarketVault {
  #[inline]
  pub fn resources(&self) -> Resources {
    self.resources
  }

  /// Adds resources to the vault.
  pub(crate) fn store(&mut self, resources: Resources) {
    self.resources += resources;
  }
}
