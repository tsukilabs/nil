// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::Stability;
use crate::infrastructure::building::BuildingLevel;
use derive_more::{Deref, Into};
use nil_num::impl_mul_ceil;
use nil_num::ops::MulCeil;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Workforce is a special resource used to construct buildings and recruit troops.
/// The amount generated per round will always be equal to the level of the relevant building.
///
/// Unlike other resources, workforce should never accumulate for the next round.
/// Anything that is not used should be discarded.
#[derive(
  Clone, Copy, Debug, Deref, Into, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[into(u32, f64)]
pub struct Workforce(u32);

impl Workforce {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<BuildingLevel> for Workforce {
  fn from(value: BuildingLevel) -> Self {
    Workforce(u32::from(value))
  }
}

impl From<f64> for Workforce {
  fn from(value: f64) -> Self {
    debug_assert!(value.is_finite());
    Self::new(value as u32)
  }
}

impl PartialEq<u32> for Workforce {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<u32> for Workforce {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Add for Workforce {
  type Output = Workforce;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl AddAssign for Workforce {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Sub for Workforce {
  type Output = Workforce;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl SubAssign for Workforce {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Mul for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: Workforce) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

impl Mul<u32> for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_mul(rhs))
  }
}

impl Mul<f64> for Workforce {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    f64::from(self.0) * rhs
  }
}

impl Mul<NonZeroU32> for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: NonZeroU32) -> Self::Output {
    self * rhs.get()
  }
}

impl Mul<Stability> for Workforce {
  type Output = Workforce;

  fn mul(self, rhs: Stability) -> Self::Output {
    Self::from(self.mul_ceil(*rhs))
  }
}

impl MulAssign for Workforce {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl MulAssign<Stability> for Workforce {
  fn mul_assign(&mut self, rhs: Stability) {
    *self = *self * rhs;
  }
}

impl_mul_ceil!(Workforce);
