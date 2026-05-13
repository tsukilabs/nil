// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::Stability;
use crate::infrastructure::building::{Building, BuildingLevel};
use crate::world::config::WorldConfig;
use nil_num::mul_ceil::MulCeil;
use nil_num::{F64Ops, impl_mul_ceil};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign, Deref, Mul, MulAssign, Sub, SubAssign};

/// Workforce is a special resource used to construct buildings and recruit troops.
/// The amount generated per round will always be equal to the level of the relevant
/// building multiplied by the world speed.
///
/// Unlike other resources, workforce should never accumulate for the next round.
/// Anything that is not used should be discarded.
#[derive(Copy, Debug, Deserialize, Serialize, F64Ops)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Workforce(u32);

impl Workforce {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl const Deref for Workforce {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl const From<u32> for Workforce {
  fn from(value: u32) -> Self {
    Workforce::new(value)
  }
}

impl const From<Workforce> for u32 {
  fn from(value: Workforce) -> Self {
    value.0
  }
}

impl const From<Workforce> for f64 {
  fn from(value: Workforce) -> Self {
    f64::from(value.0)
  }
}

impl const From<BuildingLevel> for Workforce {
  fn from(value: BuildingLevel) -> Self {
    Workforce(u32::from(value))
  }
}

impl const From<f64> for Workforce {
  fn from(value: f64) -> Self {
    debug_assert!(value >= 0.0);
    debug_assert!(value.is_finite());
    Self::new(value.ceil() as u32)
  }
}

impl const PartialEq<u32> for Workforce {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

impl const PartialOrd<u32> for Workforce {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl const Add for Workforce {
  type Output = Workforce;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl const AddAssign for Workforce {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl const Sub for Workforce {
  type Output = Workforce;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl const SubAssign for Workforce {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl const Mul for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: Workforce) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

impl const Mul<u32> for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_mul(rhs))
  }
}

impl const Mul<NonZeroU32> for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: NonZeroU32) -> Self::Output {
    self * rhs.get()
  }
}

impl const Mul<Stability> for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: Stability) -> Self::Output {
    Self::from(self.mul_ceil(*rhs))
  }
}

impl const MulAssign for Workforce {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl const MulAssign<Stability> for Workforce {
  fn mul_assign(&mut self, rhs: Stability) {
    *self = *self * rhs;
  }
}

impl_mul_ceil!(Workforce);

/// A building that generates workforce, such as the prefecture.
pub trait WorkforceSource: Building {
  fn workforce(&self, config: &WorldConfig) -> Workforce {
    Workforce::from(f64::from(self.level()) * config.speed())
  }
}
