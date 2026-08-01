// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Display;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Gold(u32);

impl Gold {
  pub const MIN: Self = Self::new(0);
  pub const MAX: Self = Self::new(u32::MAX);

  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }

  #[inline]
  pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    self.0.checked_sub(rhs.0).map(Self::new)
  }
}

const impl From<u32> for Gold {
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}

const impl From<Gold> for u32 {
  fn from(value: Gold) -> Self {
    value.0
  }
}

const impl From<f64> for Gold {
  fn from(value: f64) -> Self {
    debug_assert!(value.is_finite());
    debug_assert!(value >= 0.0);
    Self(value.trunc() as u32)
  }
}

const impl From<Gold> for f64 {
  fn from(value: Gold) -> Self {
    f64::from(value.0)
  }
}
