// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::resources::Resources;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[derive_const(Clone, Default, PartialEq, Eq)]
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

  pub(crate) fn withdraw(&mut self, resources: Resources) -> Result<()> {
    self.resources = self
      .resources
      .checked_sub(resources)
      .ok_or(Error::NotEnoughResourcesInMarketVault)?;

    Ok(())
  }

  /// Sets the resources in the vault.
  ///
  /// This should only be used to execute cheats or for testing purposes.
  pub(crate) fn set(&mut self, resources: Resources) {
    self.resources = resources;
  }
}
