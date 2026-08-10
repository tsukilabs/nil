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
  pub(crate) resources: Resources,
}

impl MarketVault {
  #[inline]
  pub fn resources(&self) -> Resources {
    self.resources
  }

  pub(crate) fn withdraw(&mut self, resources: Resources) -> Result<()> {
    self.resources = self
      .resources
      .checked_sub(resources)
      .ok_or(Error::NotEnoughResourcesInMarketVault)?;

    Ok(())
  }
}
