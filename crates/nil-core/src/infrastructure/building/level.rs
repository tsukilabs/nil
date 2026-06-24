// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{Deref, Into};
use nil_util::F64Math;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Copy, Debug, Deref, derive_more::Display, Into, Hash, Deserialize, Serialize, F64Math)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[into(i16, i32, u8, u16, u64, usize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct BuildingLevel(u8);

impl BuildingLevel {
  pub const ZERO: BuildingLevel = BuildingLevel(0);

  #[inline]
  pub const fn new(level: u8) -> Self {
    Self(level)
  }
}

const impl From<BuildingLevel> for i8 {
  fn from(value: BuildingLevel) -> Self {
    debug_assert!(i8::try_from(value.0).is_ok());
    i8::try_from(value.0).unwrap_or(i8::MAX)
  }
}

const impl From<BuildingLevel> for u32 {
  fn from(value: BuildingLevel) -> Self {
    u32::from(value.0)
  }
}

const impl From<BuildingLevel> for f64 {
  fn from(value: BuildingLevel) -> Self {
    f64::from(value.0)
  }
}

const impl PartialEq<u8> for BuildingLevel {
  fn eq(&self, other: &u8) -> bool {
    self.0.eq(other)
  }
}

const impl PartialEq<BuildingLevel> for u8 {
  fn eq(&self, other: &BuildingLevel) -> bool {
    self.eq(&other.0)
  }
}

const impl PartialEq<f64> for BuildingLevel {
  fn eq(&self, other: &f64) -> bool {
    f64::from(self.0).eq(other)
  }
}

const impl PartialEq<BuildingLevel> for f64 {
  fn eq(&self, other: &BuildingLevel) -> bool {
    self.eq(&f64::from(other.0))
  }
}

const impl PartialOrd<u8> for BuildingLevel {
  fn partial_cmp(&self, other: &u8) -> Option<cmp::Ordering> {
    self.0.partial_cmp(other)
  }
}

const impl PartialOrd<BuildingLevel> for u8 {
  fn partial_cmp(&self, other: &BuildingLevel) -> Option<cmp::Ordering> {
    self.partial_cmp(&other.0)
  }
}

const impl PartialOrd<f64> for BuildingLevel {
  fn partial_cmp(&self, other: &f64) -> Option<cmp::Ordering> {
    f64::from(self.0).partial_cmp(other)
  }
}

const impl PartialOrd<BuildingLevel> for f64 {
  fn partial_cmp(&self, other: &BuildingLevel) -> Option<cmp::Ordering> {
    self.partial_cmp(&f64::from(other.0))
  }
}

const impl Add for BuildingLevel {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self(self.0.saturating_add(rhs.0))
  }
}

const impl Add<u8> for BuildingLevel {
  type Output = Self;

  fn add(self, rhs: u8) -> Self {
    Self(self.0.saturating_add(rhs))
  }
}

const impl Add<i8> for BuildingLevel {
  type Output = Self;

  fn add(self, rhs: i8) -> Self {
    Self(self.0.saturating_add_signed(rhs))
  }
}

const impl Add<BuildingLevelDiff> for BuildingLevel {
  type Output = Self;

  fn add(self, rhs: BuildingLevelDiff) -> Self {
    self + rhs.0
  }
}

const impl AddAssign for BuildingLevel {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

const impl AddAssign<u8> for BuildingLevel {
  fn add_assign(&mut self, rhs: u8) {
    *self = *self + rhs;
  }
}

const impl AddAssign<i8> for BuildingLevel {
  fn add_assign(&mut self, rhs: i8) {
    *self = *self + rhs;
  }
}

const impl AddAssign<BuildingLevelDiff> for BuildingLevel {
  fn add_assign(&mut self, rhs: BuildingLevelDiff) {
    *self = *self + rhs;
  }
}

const impl Sub for BuildingLevel {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self(self.0.saturating_sub(rhs.0))
  }
}

const impl Sub<u8> for BuildingLevel {
  type Output = Self;

  fn sub(self, rhs: u8) -> Self {
    Self(self.0.saturating_sub(rhs))
  }
}

const impl Sub<i8> for BuildingLevel {
  type Output = Self;

  fn sub(self, rhs: i8) -> Self {
    Self(self.0.saturating_sub_signed(rhs))
  }
}

const impl Sub<BuildingLevelDiff> for BuildingLevel {
  type Output = Self;

  fn sub(self, rhs: BuildingLevelDiff) -> Self {
    self - rhs.0
  }
}

const impl SubAssign for BuildingLevel {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

const impl SubAssign<u8> for BuildingLevel {
  fn sub_assign(&mut self, rhs: u8) {
    *self = *self - rhs;
  }
}

const impl SubAssign<i8> for BuildingLevel {
  fn sub_assign(&mut self, rhs: i8) {
    *self = *self - rhs;
  }
}

const impl SubAssign<BuildingLevelDiff> for BuildingLevel {
  fn sub_assign(&mut self, rhs: BuildingLevelDiff) {
    *self = *self - rhs;
  }
}

const impl Neg for BuildingLevel {
  type Output = BuildingLevelDiff;

  fn neg(self) -> BuildingLevelDiff {
    BuildingLevelDiff::new(i8::from(self).neg())
  }
}

#[derive(Clone, Copy, Debug, Deref, derive_more::Display, Into, Hash, Deserialize, Serialize)]
#[derive_const(Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct BuildingLevelDiff(i8);

impl BuildingLevelDiff {
  pub const ZERO: BuildingLevelDiff = BuildingLevelDiff(0);

  #[inline]
  pub const fn new(level_diff: i8) -> Self {
    Self(level_diff)
  }
}

const impl From<f64> for BuildingLevelDiff {
  fn from(mut value: f64) -> Self {
    value = value.round();
    debug_assert!(value.is_finite());
    debug_assert!(value >= f64::from(i8::MIN));
    debug_assert!(value <= f64::from(i8::MAX));
    Self::new(value as i8)
  }
}

const impl PartialEq<i8> for BuildingLevelDiff {
  fn eq(&self, other: &i8) -> bool {
    self.0.eq(other)
  }
}

const impl PartialOrd<i8> for BuildingLevelDiff {
  fn partial_cmp(&self, other: &i8) -> Option<cmp::Ordering> {
    self.0.partial_cmp(other)
  }
}

/// Shorthand for [`BuildingLevel::new`].
#[macro_export]
macro_rules! lv {
  ($level:expr) => {
    const { $crate::infrastructure::building::level::BuildingLevel::new($level) }
  };
}

/// Creates a building with a random level.
#[macro_export]
macro_rules! with_random_level {
  ($building:ident) => {{ $crate::infrastructure::prelude::$building::with_random_level() }};
  ($building:ident, $max:expr) => {{
    $crate::infrastructure::prelude::$building::with_random_level_in()
      .max($crate::lv!($max))
      .call()
  }};
  ($building:ident, $min:expr, $max:expr) => {{
    $crate::infrastructure::prelude::$building::with_random_level_in()
      .min($crate::lv!($min))
      .max($crate::lv!($max))
      .call()
  }};
}
