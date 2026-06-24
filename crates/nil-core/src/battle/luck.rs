// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Display;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
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

const impl From<i8> for Luck {
  fn from(value: i8) -> Self {
    Self::new(value)
  }
}

const impl From<Luck> for f64 {
  fn from(luck: Luck) -> Self {
    f64::from(luck.0) / 100.0
  }
}
