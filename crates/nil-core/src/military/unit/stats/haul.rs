// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
#[into(u16, u32, f64)]
pub struct Haul(u16);

impl Haul {
  #[inline]
  pub const fn new(value: u16) -> Self {
    Self(value)
  }
}
