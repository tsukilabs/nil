// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Into;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Opinion(i16);

impl Opinion {
  pub const MIN: Opinion = Opinion(-1500);
  pub const MAX: Opinion = Opinion(1500);

  pub const fn new(value: i16) -> Self {
    Self(value.clamp(Self::MIN.0, Self::MAX.0))
  }
}

impl const Default for Opinion {
  fn default() -> Self {
    Self(0)
  }
}

impl const Deref for Opinion {
  type Target = i16;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
