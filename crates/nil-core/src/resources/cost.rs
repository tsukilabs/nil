// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::maintenance::MaintenanceRatio;
use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};
use std::ops::Mul;

/// Base cost of an entity, such as buildings or units.
#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
#[into(u32, f64)]
pub struct Cost(u32);

impl Cost {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<f64> for Cost {
  fn from(value: f64) -> Self {
    debug_assert!(value.is_finite());
    Self::new(value as u32)
  }
}

impl Mul<f64> for Cost {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    f64::from(self) * rhs
  }
}

impl Mul<ResourceRatio> for Cost {
  type Output = f64;

  fn mul(self, rhs: ResourceRatio) -> Self::Output {
    self * rhs.0
  }
}

impl Mul<MaintenanceRatio> for Cost {
  type Output = f64;

  fn mul(self, rhs: MaintenanceRatio) -> Self::Output {
    self * f64::from(rhs)
  }
}

/// Proportion between the total cost and a given resource.
#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct ResourceRatio(f64);

impl ResourceRatio {
  #[inline]
  pub const fn new(value: f64) -> Self {
    debug_assert!(value.is_finite());
    Self(value.clamp(0.0, 1.0))
  }

  #[inline]
  pub const fn as_f64(self) -> f64 {
    self.0
  }
}

/// Checks, at compile time, if the sum of the resource ratios equals 1.
#[macro_export]
macro_rules! check_total_resource_ratio {
  ($first:expr, $($other:expr),+ $(,)?) => {
    const _: () = {
      let mut first = $first.as_f64();
      $(first += $other.as_f64();)+
      assert!((first - 1.0).abs() < 0.001);
    };
  };
}
