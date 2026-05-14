// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_num::F64Math;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::Deref;

/// Represents how many fields a unit can travel in one round.
#[derive(Copy, Debug, Deserialize, Serialize, F64Math)]
#[derive_const(Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Speed(f64);

impl Speed {
  #[inline]
  pub const fn new(value: f64) -> Self {
    debug_assert!(value >= 0.0);
    debug_assert!(value.is_finite());
    Self(value.max(0.0))
  }
}

impl const Deref for Speed {
  type Target = f64;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl const From<Speed> for f64 {
  fn from(value: Speed) -> Self {
    value.0
  }
}

impl const PartialEq<f64> for Speed {
  fn eq(&self, other: &f64) -> bool {
    self.0.eq(other)
  }
}

impl const PartialOrd<f64> for Speed {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}
