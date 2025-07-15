// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Deref;
use serde::{Deserialize, Serialize};

/// Base cost of an entity, such as buildings or units.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct Cost(u32);

impl Cost {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<Cost> for f64 {
  fn from(value: Cost) -> Self {
    f64::from(value.0)
  }
}

impl From<f64> for Cost {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

/// Proportion between the total cost and a given resource.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct ResourceRatio(f64);

impl ResourceRatio {
  #[inline]
  pub const fn new(value: f64) -> Self {
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
