// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Food;
use super::diff::FoodDiff;
use crate::military::army::Army;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::squad::Squad;
use crate::military::unit::UnitChunkSize;
use derive_more::Display;
use nil_util::ConstDeref;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::iter::Sum;
use std::num::NonZeroU32;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// Maintenance tax of an entity.
///
/// Its value is equivalent to a percentage of the [base cost].
///
/// [base cost]: crate::resources::cost::Cost
#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Maintenance(Food);

impl Maintenance {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(Food::new(value))
  }
}

impl const From<u32> for Maintenance {
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}

impl const From<Maintenance> for u32 {
  fn from(value: Maintenance) -> Self {
    u32::from(value.0)
  }
}

impl const From<f64> for Maintenance {
  fn from(value: f64) -> Self {
    debug_assert!(value.is_finite());
    Self::new(value as u32)
  }
}

impl const From<Maintenance> for f64 {
  fn from(value: Maintenance) -> Self {
    f64::from(value.0)
  }
}

impl<'a> Sum<&'a Squad> for Maintenance {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Squad>,
  {
    iter.fold(Maintenance::default(), |mut acc, squad| {
      acc += squad.maintenance();
      acc
    })
  }
}

impl<'a> Sum<&'a ArmyPersonnel> for Maintenance {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a ArmyPersonnel>,
  {
    iter.flat_map(ArmyPersonnel::iter).sum()
  }
}

impl<'a> Sum<&'a Army> for Maintenance {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Army>,
  {
    iter.flat_map(Army::iter).sum()
  }
}

impl const Add for Maintenance {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self(self.0 + rhs.0)
  }
}

impl const Add<Food> for Maintenance {
  type Output = Self;

  fn add(self, rhs: Food) -> Self {
    Self(self.0 + rhs)
  }
}

impl const AddAssign for Maintenance {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self(self.0 + rhs.0);
  }
}

impl const AddAssign<Food> for Maintenance {
  fn add_assign(&mut self, rhs: Food) {
    *self = Self(self.0 + rhs);
  }
}

impl const Sub for Maintenance {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self(self.0 - rhs.0)
  }
}

impl const Sub<Food> for Maintenance {
  type Output = Self;

  fn sub(self, rhs: Food) -> Self {
    Self(self.0 - rhs)
  }
}

impl const Sub<Maintenance> for Food {
  type Output = Self;

  fn sub(self, rhs: Maintenance) -> Self {
    self - rhs.0
  }
}

impl const Sub<Maintenance> for FoodDiff {
  type Output = Self;

  fn sub(self, rhs: Maintenance) -> Self {
    self - rhs.0
  }
}

impl const SubAssign for Maintenance {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self(self.0 - rhs.0);
  }
}

impl const SubAssign<Food> for Maintenance {
  fn sub_assign(&mut self, rhs: Food) {
    *self = Self(self.0 - rhs);
  }
}

impl const SubAssign<Maintenance> for Food {
  fn sub_assign(&mut self, rhs: Maintenance) {
    *self = *self - rhs.0;
  }
}

impl const SubAssign<Maintenance> for FoodDiff {
  fn sub_assign(&mut self, rhs: Maintenance) {
    *self = *self - rhs.0;
  }
}

impl const Mul<u32> for Maintenance {
  type Output = Maintenance;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0 * rhs)
  }
}

impl const Mul<NonZeroU32> for Maintenance {
  type Output = Maintenance;

  fn mul(self, rhs: NonZeroU32) -> Self::Output {
    Self(self.0 * rhs.get())
  }
}

impl const Div<UnitChunkSize> for Maintenance {
  type Output = Maintenance;

  fn div(self, rhs: UnitChunkSize) -> Self::Output {
    let maintenance = f64::from(self);
    let chunk_size = f64::from(rhs);
    Self::from(maintenance / chunk_size)
  }
}

impl const PartialEq<Food> for Maintenance {
  fn eq(&self, other: &Food) -> bool {
    self.0.eq(other)
  }
}

impl const PartialEq<Maintenance> for Food {
  fn eq(&self, other: &Maintenance) -> bool {
    self.eq(&other.0)
  }
}

impl const PartialOrd<Food> for Maintenance {
  fn partial_cmp(&self, other: &Food) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl const PartialOrd<Maintenance> for Food {
  fn partial_cmp(&self, other: &Maintenance) -> Option<Ordering> {
    self.partial_cmp(&other.0)
  }
}

/// Proportion of the base cost that should be used as a maintenance tax.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, ConstDeref)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct MaintenanceRatio(f64);

impl MaintenanceRatio {
  #[inline]
  pub const fn new(ratio: f64) -> Self {
    debug_assert!(ratio.is_finite());
    debug_assert!(!ratio.is_subnormal());
    Self(ratio.clamp(0.0, 1.0))
  }
}

impl const From<MaintenanceRatio> for f64 {
  fn from(value: MaintenanceRatio) -> Self {
    value.0
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[derive_const(Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct MaintenanceBalance {
  pub maintenance: Maintenance,
  pub production: Food,
}

impl MaintenanceBalance {
  /// Checks if the food production is enough to cover the maintenance costs.
  #[inline]
  pub const fn is_sustainable(&self) -> bool {
    self.production >= self.maintenance
  }
}

impl const Add<Maintenance> for MaintenanceBalance {
  type Output = Self;

  fn add(self, rhs: Maintenance) -> Self::Output {
    Self {
      maintenance: self.maintenance + rhs,
      production: self.production,
    }
  }
}
