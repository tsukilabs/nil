// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Distance;
use crate::military::unit::stats::speed::Speed;
use nil_num::F64Ops;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Deref, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, F64Ops)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ManeuverDistance(f64);

impl const Deref for ManeuverDistance {
  type Target = f64;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl const From<ManeuverDistance> for f64 {
  fn from(value: ManeuverDistance) -> Self {
    value.0
  }
}

impl const From<Distance> for ManeuverDistance {
  fn from(distance: Distance) -> Self {
    Self(f64::from(distance))
  }
}

impl const PartialEq for ManeuverDistance {
  fn eq(&self, other: &Self) -> bool {
    matches!(self.0.total_cmp(&other.0), Ordering::Equal)
  }
}

impl const Eq for ManeuverDistance {}

impl const PartialOrd for ManeuverDistance {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl const Ord for ManeuverDistance {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.total_cmp(&other.0)
  }
}

impl const PartialEq<f64> for ManeuverDistance {
  fn eq(&self, other: &f64) -> bool {
    matches!(self.0.total_cmp(other), Ordering::Equal)
  }
}

impl const PartialOrd<f64> for ManeuverDistance {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    Some(self.0.total_cmp(other))
  }
}

impl const Sub for ManeuverDistance {
  type Output = ManeuverDistance;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self -= rhs;
    self
  }
}

impl const Sub<Speed> for ManeuverDistance {
  type Output = ManeuverDistance;

  fn sub(mut self, rhs: Speed) -> Self::Output {
    self -= rhs;
    self
  }
}

impl const SubAssign for ManeuverDistance {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self(self.0 - rhs.0);
  }
}

impl const SubAssign<Speed> for ManeuverDistance {
  fn sub_assign(&mut self, rhs: Speed) {
    *self = Self(self.0 - f64::from(rhs));
  }
}
