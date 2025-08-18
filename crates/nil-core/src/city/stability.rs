// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};

/// Political stability of the city.
#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct Stability(f64);

impl Stability {
  pub const MIN: Stability = Stability(0.0);
  pub const MAX: Stability = Stability(1.0);

  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(Self::MIN.0, Self::MAX.0))
  }
}

impl Default for Stability {
  fn default() -> Self {
    Self::MAX
  }
}
