// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Display;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Market {
  fee: MarketFee,
}

impl Market {
  pub fn new(fee: MarketFee) -> Self {
    Self { fee }
  }

  #[inline]
  pub fn fee(&self) -> MarketFee {
    self.fee
  }

  pub(crate) fn fee_mut(&mut self) -> &mut MarketFee {
    &mut self.fee
  }
}

#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct MarketFee(f64);

impl MarketFee {
  pub const MIN: MarketFee = MarketFee(0.05);
  pub const MAX: MarketFee = MarketFee(1.0);

  #[inline]
  pub const fn new(value: f64) -> Self {
    debug_assert!(value.is_finite());
    debug_assert!(!value.is_subnormal());
    Self(value.clamp(Self::MIN.0, Self::MAX.0))
  }

  #[inline]
  pub fn clamp(value: &mut Self) {
    *value = Self::new(value.0);
  }
}

const impl Default for MarketFee {
  fn default() -> Self {
    Self::new(0.3)
  }
}

const impl From<f64> for MarketFee {
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}

const impl From<MarketFee> for f64 {
  fn from(value: MarketFee) -> Self {
    value.0
  }
}
