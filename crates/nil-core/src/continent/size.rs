// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::Into;
use nil_util::ConstDeref;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::num::NonZeroU8;

#[derive(Clone, Copy, Debug, Into, Deserialize, Serialize, ConstDeref)]
#[derive_const(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ContinentSize(NonZeroU8);

impl ContinentSize {
  pub const MIN: ContinentSize = unsafe { Self::new_unchecked(100) };
  pub const MAX: ContinentSize = unsafe { Self::new_unchecked(200) };

  pub const fn new(size: u8) -> Self {
    let size = size
      .clamp(Self::MIN.0.get(), Self::MAX.0.get())
      .next_multiple_of(10);

    unsafe { Self::new_unchecked(size) }
  }

  /// # Safety
  ///
  /// The size must be between [`ContinentSize::MIN`] and [`ContinentSize::MAX`].
  #[inline]
  pub const unsafe fn new_unchecked(size: u8) -> Self {
    Self(unsafe { NonZeroU8::new_unchecked(size) })
  }
}

impl const Default for ContinentSize {
  fn default() -> Self {
    Self::MIN
  }
}

impl const From<ContinentSize> for u8 {
  fn from(size: ContinentSize) -> Self {
    size.0.get()
  }
}

impl const From<ContinentSize> for u16 {
  fn from(size: ContinentSize) -> Self {
    u16::from(size.0.get())
  }
}

impl const From<ContinentSize> for usize {
  fn from(size: ContinentSize) -> Self {
    usize::from(size.0.get())
  }
}

impl const From<ContinentSize> for i16 {
  fn from(size: ContinentSize) -> Self {
    i16::from(size.0.get())
  }
}

impl const From<ContinentSize> for f64 {
  fn from(size: ContinentSize) -> Self {
    f64::from(size.0.get())
  }
}

impl const PartialEq<u8> for ContinentSize {
  fn eq(&self, other: &u8) -> bool {
    self.0.get().eq(other)
  }
}

impl const PartialEq<usize> for ContinentSize {
  fn eq(&self, other: &usize) -> bool {
    usize::from(self.0.get()).eq(other)
  }
}

impl const PartialEq<ContinentSize> for usize {
  fn eq(&self, other: &ContinentSize) -> bool {
    self.eq(&usize::from(other.0.get()))
  }
}

impl const PartialOrd<u8> for ContinentSize {
  fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
    self.0.get().partial_cmp(other)
  }
}

impl const PartialOrd<usize> for ContinentSize {
  fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
    usize::from(self.0.get()).partial_cmp(other)
  }
}

impl const PartialOrd<ContinentSize> for usize {
  fn partial_cmp(&self, other: &ContinentSize) -> Option<Ordering> {
    self.partial_cmp(&usize::from(other.0.get()))
  }
}
