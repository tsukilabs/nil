// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Food;
use super::diff::FoodDiff;
use derive_more::{Deref, Display, Into};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Maintenance tax of an entity.
///
/// Its value is equivalent to a percentage of the [base cost].
///
/// [base cost]: crate::resource::cost::Cost
#[derive(Clone, Copy, Debug, Default, Deref, Display, Into, Deserialize, Serialize)]
#[into(u32, f64, Food)]
pub struct Maintenance(Food);

impl Maintenance {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(Food::new(value))
  }
}

impl From<u32> for Maintenance {
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}

impl From<f64> for Maintenance {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

impl Add for Maintenance {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self(self.0 + rhs.0)
  }
}

impl Add<Food> for Maintenance {
  type Output = Self;

  fn add(self, rhs: Food) -> Self {
    Self(self.0 + rhs)
  }
}

impl AddAssign for Maintenance {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self(self.0 + rhs.0);
  }
}

impl AddAssign<Food> for Maintenance {
  fn add_assign(&mut self, rhs: Food) {
    *self = Self(self.0 + rhs);
  }
}

impl Sub for Maintenance {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self(self.0 - rhs.0)
  }
}

impl Sub<Food> for Maintenance {
  type Output = Self;

  fn sub(self, rhs: Food) -> Self {
    Self(self.0 - rhs)
  }
}

impl Sub<Maintenance> for Food {
  type Output = Self;

  fn sub(self, rhs: Maintenance) -> Self {
    self - rhs.0
  }
}

impl Sub<Maintenance> for FoodDiff {
  type Output = Self;

  fn sub(self, rhs: Maintenance) -> Self {
    self - rhs.0
  }
}

impl SubAssign for Maintenance {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self(self.0 - rhs.0);
  }
}

impl SubAssign<Food> for Maintenance {
  fn sub_assign(&mut self, rhs: Food) {
    *self = Self(self.0 - rhs);
  }
}

impl SubAssign<Maintenance> for Food {
  fn sub_assign(&mut self, rhs: Maintenance) {
    *self = *self - rhs.0;
  }
}

impl SubAssign<Maintenance> for FoodDiff {
  fn sub_assign(&mut self, rhs: Maintenance) {
    *self = *self - rhs.0;
  }
}

/// Proportion of the base cost that should be used as a maintenance tax.
#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct MaintenanceRatio(f64);

impl MaintenanceRatio {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(0.0, 1.0))
  }
}
