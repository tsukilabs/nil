// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Into;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Debug, Into)]
#[derive_const(Clone)]
pub struct BehaviorScore(f64);

impl BehaviorScore {
  pub const MIN: Self = BehaviorScore(0.0);
  pub const MAX: Self = BehaviorScore(1.0);

  #[inline]
  pub const fn new(score: f64) -> Self {
    debug_assert!(score.is_finite());
    debug_assert!(!score.is_subnormal());
    Self(score.clamp(Self::MIN.0, Self::MAX.0))
  }

  #[inline]
  pub const fn is_within_range(self, other: BehaviorScore, range: f64) -> bool {
    (self.0 - other.0).abs() < range
  }
}

const impl Default for BehaviorScore {
  fn default() -> Self {
    Self::MIN
  }
}

const impl From<f64> for BehaviorScore {
  fn from(score: f64) -> Self {
    Self::new(score)
  }
}

const impl PartialEq for BehaviorScore {
  fn eq(&self, other: &Self) -> bool {
    matches!(self.0.total_cmp(&other.0), Ordering::Equal)
  }
}

const impl Eq for BehaviorScore {}

const impl PartialOrd for BehaviorScore {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

const impl Ord for BehaviorScore {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.total_cmp(&other.0)
  }
}

const impl PartialEq<f64> for BehaviorScore {
  fn eq(&self, other: &f64) -> bool {
    self.0.eq(other)
  }
}

const impl PartialOrd<f64> for BehaviorScore {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

const impl Add<f64> for BehaviorScore {
  type Output = BehaviorScore;

  fn add(self, rhs: f64) -> Self::Output {
    BehaviorScore::new(self.0 + rhs)
  }
}

const impl AddAssign<f64> for BehaviorScore {
  fn add_assign(&mut self, rhs: f64) {
    *self = *self + rhs;
  }
}

const impl Sub<f64> for BehaviorScore {
  type Output = BehaviorScore;

  fn sub(self, rhs: f64) -> Self::Output {
    BehaviorScore::new(self.0 - rhs)
  }
}

const impl SubAssign<f64> for BehaviorScore {
  fn sub_assign(&mut self, rhs: f64) {
    *self = *self - rhs;
  }
}

const impl Mul<f64> for BehaviorScore {
  type Output = BehaviorScore;

  fn mul(self, rhs: f64) -> Self::Output {
    BehaviorScore::new(self.0 * rhs)
  }
}

const impl MulAssign<f64> for BehaviorScore {
  fn mul_assign(&mut self, rhs: f64) {
    *self = *self * rhs;
  }
}

const impl Div<f64> for BehaviorScore {
  type Output = BehaviorScore;

  fn div(self, rhs: f64) -> Self::Output {
    BehaviorScore::new(self.0 / rhs)
  }
}

const impl DivAssign<f64> for BehaviorScore {
  fn div_assign(&mut self, rhs: f64) {
    *self = *self / rhs;
  }
}
