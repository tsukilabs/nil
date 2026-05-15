// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Copy, Debug, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct RangedDebuff(f64);

impl RangedDebuff {
  pub const MIN: RangedDebuff = RangedDebuff(0.0);

  #[inline]
  pub const fn new(value: f64) -> Self {
    debug_assert!(value.is_finite());
    Self(value.max(Self::MIN.0))
  }
}

impl const From<RangedDebuff> for f64 {
  fn from(value: RangedDebuff) -> Self {
    value.0
  }
}

impl const Mul<u32> for RangedDebuff {
  type Output = f64;

  fn mul(self, rhs: u32) -> Self::Output {
    self.0 * f64::from(rhs)
  }
}
