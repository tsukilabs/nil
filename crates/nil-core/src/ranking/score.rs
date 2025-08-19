// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Into};
use nil_num::impl_mul_ceil;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(
  Clone, Copy, Debug, Default, Deref, Into, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[into(u32, f64)]
pub struct Score(u32);

impl Score {
  pub const ZERO: Score = Score(0);

  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<f64> for Score {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

impl PartialEq<u32> for Score {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<u32> for Score {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Add for Score {
  type Output = Score;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl AddAssign for Score {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Add<u32> for Score {
  type Output = Score;

  fn add(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_add(rhs))
  }
}

impl AddAssign<u32> for Score {
  fn add_assign(&mut self, rhs: u32) {
    *self = *self + rhs;
  }
}

impl Sub for Score {
  type Output = Score;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl SubAssign for Score {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Mul for Score {
  type Output = Score;

  fn mul(self, rhs: Score) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

impl Mul<u32> for Score {
  type Output = Score;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_mul(rhs))
  }
}

impl Mul<f64> for Score {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    f64::from(self.0) * rhs
  }
}

impl MulAssign for Score {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl_mul_ceil!(Score);
