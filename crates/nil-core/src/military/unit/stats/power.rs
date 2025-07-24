// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};
use std::ops::{Div, Mul};

#[derive(Clone, Copy, Debug, Deref, Into, PartialEq, Eq, Deserialize, Serialize)]
#[into(u32, f64)]
pub struct Power(u32);

impl Power {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl Mul for Power {
  type Output = Power;

  fn mul(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

impl Mul<u32> for Power {
  type Output = Power;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_mul(rhs))
  }
}

impl Mul<Power> for u32 {
  type Output = u32;

  fn mul(self, rhs: Power) -> Self::Output {
    self.saturating_mul(rhs.0)
  }
}

impl Div for Power {
  type Output = Power;

  fn div(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_div(rhs.0))
  }
}

impl Div<u32> for Power {
  type Output = Power;

  fn div(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_div(rhs))
  }
}

impl Div<Power> for u32 {
  type Output = u32;

  fn div(self, rhs: Power) -> Self::Output {
    self.saturating_div(rhs.0)
  }
}
