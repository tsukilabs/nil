// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::unit::stats::power::Power;
use crate::ranking::score::Score;
use crate::resources::maintenance::Maintenance;
use derive_more::{Display, From, Into};
use nil_util::ConstDeref;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Debug, Display, From, Into, Deserialize, Serialize, ConstDeref)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[into(u32, f64)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct SquadSize(u32);

impl SquadSize {
  #[inline]
  pub const fn new(size: u32) -> Self {
    Self(size)
  }

  #[inline]
  pub fn random() -> Self {
    Self::new(rand::random())
  }

  #[inline]
  pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    self.0.checked_sub(rhs.0).map(Self::new)
  }
}

impl const PartialEq<u32> for SquadSize {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

impl const PartialOrd<u32> for SquadSize {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl const Add for SquadSize {
  type Output = SquadSize;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl const Add<u32> for SquadSize {
  type Output = SquadSize;

  fn add(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_add(rhs))
  }
}

impl const AddAssign for SquadSize {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl const AddAssign<u32> for SquadSize {
  fn add_assign(&mut self, rhs: u32) {
    *self = *self + rhs;
  }
}

impl const From<f64> for SquadSize {
  fn from(value: f64) -> Self {
    debug_assert!(value >= 0.0);
    debug_assert!(value.is_finite());
    Self::new(value as u32)
  }
}

impl const Sub for SquadSize {
  type Output = SquadSize;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl const Sub<u32> for SquadSize {
  type Output = SquadSize;

  fn sub(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_sub(rhs))
  }
}

impl const SubAssign for SquadSize {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl const SubAssign<u32> for SquadSize {
  fn sub_assign(&mut self, rhs: u32) {
    *self = *self - rhs;
  }
}

impl const Mul<Maintenance> for SquadSize {
  type Output = Maintenance;

  fn mul(self, rhs: Maintenance) -> Self::Output {
    let rhs = u32::from(rhs);
    Maintenance::new(self.0.saturating_mul(rhs))
  }
}

impl const Mul<Power> for SquadSize {
  type Output = Power;

  fn mul(self, rhs: Power) -> Self::Output {
    rhs * self.0
  }
}

impl const Mul<SquadSize> for Power {
  type Output = Power;

  fn mul(self, rhs: SquadSize) -> Self::Output {
    self * rhs.0
  }
}

impl const Mul<Score> for SquadSize {
  type Output = Score;

  fn mul(self, rhs: Score) -> Self::Output {
    let rhs = u32::from(rhs);
    Score::new(self.0.saturating_mul(rhs))
  }
}

impl const Mul<f64> for SquadSize {
  type Output = SquadSize;

  fn mul(self, rhs: f64) -> Self::Output {
    debug_assert!(rhs.is_finite());
    debug_assert!(rhs.is_sign_positive());
    debug_assert!(!rhs.is_subnormal());
    Self((f64::from(self.0) * rhs).floor() as u32)
  }
}

impl const MulAssign<f64> for SquadSize {
  fn mul_assign(&mut self, rhs: f64) {
    *self = *self * rhs;
  }
}
