// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod fee;

use crate::market::fee::MarketFee;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Market {
  fee: MarketFee,
}

impl Market {
  pub fn new(fee: MarketFee) -> Self {
    Self { fee: fee.clamped() }
  }

  #[inline]
  pub fn fee(&self) -> MarketFee {
    self.fee
  }

  pub(crate) fn fee_mut(&mut self) -> &mut MarketFee {
    &mut self.fee
  }
}
