// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::maneuver::distance::ManeuverDistance;
use derive_more::Display;
use nil_util::ConstDeref;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Distance(u8);

impl Distance {
  #[inline]
  pub const fn new(distance: u8) -> Self {
    Self(distance)
  }
}

const impl From<u8> for Distance {
  fn from(value: u8) -> Self {
    Self(value)
  }
}

const impl From<Distance> for u8 {
  fn from(value: Distance) -> Self {
    value.0
  }
}

const impl From<Distance> for i16 {
  fn from(value: Distance) -> Self {
    i16::from(value.0)
  }
}

const impl From<Distance> for f64 {
  fn from(value: Distance) -> Self {
    f64::from(value.0)
  }
}

const impl PartialEq<u8> for Distance {
  fn eq(&self, other: &u8) -> bool {
    self.0.eq(other)
  }
}

const impl Add for Distance {
  type Output = Distance;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

const impl AddAssign for Distance {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

const impl Sub for Distance {
  type Output = Distance;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

const impl SubAssign for Distance {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

const impl Add<u8> for Distance {
  type Output = Distance;

  fn add(self, rhs: u8) -> Self::Output {
    Self(self.0.saturating_add(rhs))
  }
}

const impl AddAssign<u8> for Distance {
  fn add_assign(&mut self, rhs: u8) {
    *self = *self + rhs;
  }
}

const impl Sub<u8> for Distance {
  type Output = Distance;

  fn sub(self, rhs: u8) -> Self::Output {
    Self(self.0.saturating_sub(rhs))
  }
}

const impl Sub<ManeuverDistance> for Distance {
  type Output = ManeuverDistance;

  fn sub(self, rhs: ManeuverDistance) -> Self::Output {
    ManeuverDistance::from(f64::from(self) - f64::from(rhs))
  }
}

const impl SubAssign<u8> for Distance {
  fn sub_assign(&mut self, rhs: u8) {
    *self = *self - rhs;
  }
}
