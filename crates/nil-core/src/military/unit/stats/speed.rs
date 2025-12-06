// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Represents how many fields a unit can travel in one round.
#[derive(Clone, Copy, Debug, Deref, Into, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Speed(f64);

impl Speed {
  #[inline]
  pub const fn new(value: f64) -> Self {
    debug_assert!(value.is_finite() && value >= 0.0);
    Self(value.max(0.0))
  }
}

impl Default for Speed {
  fn default() -> Self {
    Self(0.0)
  }
}

impl PartialEq<f64> for Speed {
  fn eq(&self, other: &f64) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<f64> for Speed {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}
