// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};

/// Represents how many fields a unit can travel in one round.
#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct Speed(f64);

impl Speed {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.max(0.0))
  }
}
