// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod fee;
pub mod vault;

use crate::market::fee::MarketFee;
use crate::market::vault::MarketVault;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Market {
  vault: MarketVault,
  fee: MarketFee,
}

impl Market {
  pub fn new(fee: MarketFee) -> Self {
    Self {
      vault: MarketVault::default(),
      fee: fee.clamped(),
    }
  }

  #[inline]
  pub fn vault(&self) -> &MarketVault {
    &self.vault
  }

  pub(crate) fn vault_mut(&mut self) -> &mut MarketVault {
    &mut self.vault
  }

  #[inline]
  pub fn fee(&self) -> MarketFee {
    self.fee
  }

  /// Sets the market fee, clamping it to the valid range.
  ///
  /// As the fee is not expected to be changed throughout the game,
  /// this should only be used to execute cheats or for testing purposes.
  pub(crate) fn set_fee(&mut self, fee: MarketFee) {
    self.fee = fee.clamped();
  }
}
