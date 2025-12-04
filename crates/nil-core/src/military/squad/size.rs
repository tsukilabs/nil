// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::unit::stats::power::Power;
use crate::ranking::Score;
use crate::resources::Maintenance;
use derive_more::{Deref, From, Into};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(
  Clone,
  Copy,
  Debug,
  Default,
  Deref,
  From,
  Into,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Deserialize,
  Serialize,
)]
#[into(u32, f64)]
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
  pub fn checked_sub(self, rhs: Self) -> Option<Self> {
    self.0.checked_sub(rhs.0).map(Self::new)
  }
}

impl PartialEq<u32> for SquadSize {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<u32> for SquadSize {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Add for SquadSize {
  type Output = SquadSize;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl Add<u32> for SquadSize {
  type Output = SquadSize;

  fn add(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_add(rhs))
  }
}

impl AddAssign for SquadSize {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl AddAssign<u32> for SquadSize {
  fn add_assign(&mut self, rhs: u32) {
    *self = *self + rhs;
  }
}

impl From<f64> for SquadSize {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

impl Sub for SquadSize {
  type Output = SquadSize;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl Sub<u32> for SquadSize {
  type Output = SquadSize;

  fn sub(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_sub(rhs))
  }
}

impl SubAssign for SquadSize {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl SubAssign<u32> for SquadSize {
  fn sub_assign(&mut self, rhs: u32) {
    *self = *self - rhs;
  }
}

impl Mul<Maintenance> for SquadSize {
  type Output = Maintenance;

  fn mul(self, rhs: Maintenance) -> Self::Output {
    let rhs = u32::from(rhs);
    Maintenance::new(self.0.saturating_mul(rhs))
  }
}

impl Mul<Power> for SquadSize {
  type Output = Power;

  fn mul(self, rhs: Power) -> Self::Output {
    rhs * self.0
  }
}

impl Mul<SquadSize> for Power {
  type Output = Power;

  fn mul(self, rhs: SquadSize) -> Self::Output {
    self * rhs.0
  }
}

impl Mul<Score> for SquadSize {
  type Output = Score;

  fn mul(self, rhs: Score) -> Self::Output {
    let rhs = u32::from(rhs);
    Score::new(self.0.saturating_mul(rhs))
  }
}
