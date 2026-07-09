// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::distance::Distance;
use crate::military::unit::stats::speed::Speed;
use derive_more::Display;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Sub, SubAssign};

#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ManeuverDistance(f64);

const impl From<ManeuverDistance> for f64 {
  fn from(value: ManeuverDistance) -> Self {
    value.0
  }
}

const impl From<f64> for ManeuverDistance {
  fn from(distance: f64) -> Self {
    Self(distance)
  }
}

const impl From<Distance> for ManeuverDistance {
  fn from(distance: Distance) -> Self {
    Self(f64::from(distance))
  }
}

const impl PartialEq for ManeuverDistance {
  fn eq(&self, other: &Self) -> bool {
    matches!(self.0.total_cmp(&other.0), Ordering::Equal)
  }
}

const impl Eq for ManeuverDistance {}

const impl PartialOrd for ManeuverDistance {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

const impl Ord for ManeuverDistance {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.total_cmp(&other.0)
  }
}

const impl PartialEq<f64> for ManeuverDistance {
  fn eq(&self, other: &f64) -> bool {
    matches!(self.0.total_cmp(other), Ordering::Equal)
  }
}

const impl PartialOrd<f64> for ManeuverDistance {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    Some(self.0.total_cmp(other))
  }
}

const impl Sub for ManeuverDistance {
  type Output = ManeuverDistance;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self -= rhs;
    self
  }
}

const impl Sub<Speed> for ManeuverDistance {
  type Output = ManeuverDistance;

  fn sub(mut self, rhs: Speed) -> Self::Output {
    self -= rhs;
    self
  }
}

const impl SubAssign for ManeuverDistance {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self(self.0 - rhs.0);
  }
}

const impl SubAssign<Speed> for ManeuverDistance {
  fn sub_assign(&mut self, rhs: Speed) {
    *self = Self(self.0 - f64::from(rhs));
  }
}
