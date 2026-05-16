// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Luck(i8);

impl Luck {
  pub const MIN: Luck = Luck(-20);
  pub const MAX: Luck = Luck(20);

  #[inline]
  pub const fn new(value: i8) -> Self {
    Self(value.clamp(Self::MIN.0, Self::MAX.0))
  }

  pub fn random() -> Self {
    Self(rand::random_range(Self::MIN.0..=Self::MAX.0))
  }
}

impl const From<i8> for Luck {
  fn from(value: i8) -> Self {
    Self::new(value)
  }
}

impl const From<Luck> for f64 {
  fn from(luck: Luck) -> Self {
    f64::from(luck.0) / 100.0
  }
}
