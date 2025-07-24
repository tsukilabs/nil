// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct RangedDebuff(f64);

impl RangedDebuff {
  pub const MIN: RangedDebuff = RangedDebuff(0.0);

  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.max(Self::MIN.0))
  }
}

impl Mul<f64> for RangedDebuff {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    self.0 * rhs
  }
}

impl Mul<u32> for RangedDebuff {
  type Output = f64;

  fn mul(self, rhs: u32) -> Self::Output {
    self.0 * f64::from(rhs)
  }
}
