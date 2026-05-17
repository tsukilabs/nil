// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::maintenance::MaintenanceRatio;
use derive_more::{Display, Into};
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::ops::Mul;

/// Base cost of an entity, such as buildings or units.
#[derive(Copy, Debug, Display, Into, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Cost(u32);

impl Cost {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl const From<f64> for Cost {
  fn from(value: f64) -> Self {
    debug_assert!(value.is_finite());
    debug_assert!(value >= 0.0);
    Self::new(value as u32)
  }
}

impl const From<Cost> for f64 {
  fn from(value: Cost) -> Self {
    f64::from(value.0)
  }
}

impl const Mul<ResourceRatio> for Cost {
  type Output = f64;

  fn mul(self, rhs: ResourceRatio) -> Self::Output {
    self * rhs.0
  }
}

impl const Mul<MaintenanceRatio> for Cost {
  type Output = f64;

  fn mul(self, rhs: MaintenanceRatio) -> Self::Output {
    self * f64::from(rhs)
  }
}

/// Proportion between the total cost and a given resource.
#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ResourceRatio(f64);

impl ResourceRatio {
  #[inline]
  pub const fn new(value: f64) -> Self {
    debug_assert!(value.is_finite());
    Self(value.clamp(0.0, 1.0))
  }
}

impl const From<ResourceRatio> for f64 {
  fn from(value: ResourceRatio) -> Self {
    value.0
  }
}

/// Checks, at compile time, if the sum of the resource ratios equals 1.
#[doc(hidden)]
#[macro_export]
macro_rules! check_total_resource_ratio {
  ($first:expr, $($other:expr),+ $(,)?) => {
    const _: () = {
      let mut first = f64::from($first);
      $(first += f64::from($other);)+
      assert!((first - 1.0).abs() < 0.001);
    };
  };
}
