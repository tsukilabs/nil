// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::ops::{Mul, MulAssign};

#[derive(Clone, Copy, Debug, Deref, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Luck(i8);

impl Luck {
  pub const MIN: Luck = Luck(-20);
  pub const MAX: Luck = Luck(20);

  #[inline]
  pub fn new(value: i8) -> Self {
    Self(value.clamp(Self::MIN.0, Self::MAX.0))
  }

  pub fn random() -> Self {
    Self(rand::random_range(Self::MIN.0..=Self::MAX.0))
  }
}

impl Default for Luck {
  fn default() -> Self {
    Self::random()
  }
}

impl From<Luck> for f64 {
  fn from(luck: Luck) -> Self {
    f64::from(luck.0) / 100.0
  }
}

impl Mul<Luck> for f64 {
  type Output = f64;

  fn mul(self, rhs: Luck) -> Self::Output {
    self * f64::from(rhs)
  }
}

impl MulAssign<Luck> for f64 {
  fn mul_assign(&mut self, rhs: Luck) {
    *self = *self * rhs;
  }
}
