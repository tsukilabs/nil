// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::From;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Debug, From, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Power(u32);

impl Power {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl const From<Power> for u32 {
  fn from(value: Power) -> Self {
    value.0
  }
}

impl const From<Power> for f64 {
  fn from(value: Power) -> Self {
    f64::from(value.0)
  }
}

impl const Add for Power {
  type Output = Power;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl const Sub for Power {
  type Output = Power;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl const Mul for Power {
  type Output = Power;

  fn mul(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

impl const Mul<u32> for Power {
  type Output = Power;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_mul(rhs))
  }
}

impl const Mul<Power> for u32 {
  type Output = u32;

  fn mul(self, rhs: Power) -> Self::Output {
    self.saturating_mul(rhs.0)
  }
}

impl const Div for Power {
  type Output = Power;

  fn div(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_div(rhs.0))
  }
}

impl const Div<u32> for Power {
  type Output = Power;

  fn div(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_div(rhs))
  }
}

impl const Div<Power> for u32 {
  type Output = u32;

  fn div(self, rhs: Power) -> Self::Output {
    self.saturating_div(rhs.0)
  }
}
