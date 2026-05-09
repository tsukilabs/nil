// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Political stability of the city.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[derive_const(PartialEq, PartialOrd)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Stability(f64);

impl Stability {
  pub const MIN: Stability = Stability(0.0);
  pub const MAX: Stability = Stability(1.0);

  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(Self::MIN.0, Self::MAX.0))
  }
}

impl const Default for Stability {
  fn default() -> Self {
    Self::MAX
  }
}

impl const Deref for Stability {
  type Target = f64;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl const From<Stability> for f64 {
  fn from(value: Stability) -> Self {
    value.0
  }
}
