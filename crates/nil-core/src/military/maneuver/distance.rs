// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Distance;
use crate::military::unit::stats::speed::Speed;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Sub, SubAssign};

#[derive(Clone, Copy, Debug, Deref, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ManeuverDistance(f64);

impl PartialEq<f64> for ManeuverDistance {
  fn eq(&self, other: &f64) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<f64> for ManeuverDistance {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Sub for ManeuverDistance {
  type Output = ManeuverDistance;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self -= rhs;
    self
  }
}

impl Sub<Speed> for ManeuverDistance {
  type Output = ManeuverDistance;

  fn sub(mut self, rhs: Speed) -> Self::Output {
    self -= rhs;
    self
  }
}

impl SubAssign for ManeuverDistance {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self(self.0 - rhs.0);
  }
}

impl SubAssign<Speed> for ManeuverDistance {
  fn sub_assign(&mut self, rhs: Speed) {
    *self = Self(self.0 - f64::from(rhs));
  }
}

impl From<Distance> for ManeuverDistance {
  fn from(distance: Distance) -> Self {
    Self(f64::from(distance))
  }
}
