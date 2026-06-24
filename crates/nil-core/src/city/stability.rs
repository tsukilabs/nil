// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Display;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};

/// Political stability of the city.
#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Stability(f64);

impl Stability {
  pub const MIN: Stability = Stability(0.0);
  pub const MAX: Stability = Stability(1.0);

  #[inline]
  pub const fn new(value: f64) -> Self {
    debug_assert!(value.is_finite());
    debug_assert!(!value.is_subnormal());
    Self(value.clamp(Self::MIN.0, Self::MAX.0))
  }
}

const impl Default for Stability {
  fn default() -> Self {
    Self::MAX
  }
}

const impl From<f64> for Stability {
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}

const impl From<Stability> for f64 {
  fn from(value: Stability) -> Self {
    value.0
  }
}
