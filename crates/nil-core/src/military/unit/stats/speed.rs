// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Display;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Represents how many fields a unit can travel in one round.
#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
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

const impl From<Speed> for f64 {
  fn from(value: Speed) -> Self {
    value.0
  }
}

const impl PartialEq<f64> for Speed {
  fn eq(&self, other: &f64) -> bool {
    self.0.eq(other)
  }
}

const impl PartialOrd<f64> for Speed {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}
