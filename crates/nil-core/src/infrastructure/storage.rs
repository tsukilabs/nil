// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::infrastructure::building::{Building, BuildingLevel, StorageId};
use derive_more::{Deref, From, Into};
use nil_num::growth::growth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A building that stores resources.
pub trait Storage: Building {
  fn storage_id(&self) -> StorageId;
  /// Storage capacity at the **current** level.
  fn capacity(&self, stats: &StorageStatsTable) -> Result<StorageCapacity>;
  /// Storage capacity at its **minimum** level.
  fn min_capacity(&self) -> StorageCapacity;
  /// Storage capacity at its **maximum** level.
  fn max_capacity(&self) -> StorageCapacity;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageStats {
  pub level: BuildingLevel,
  pub capacity: StorageCapacity,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageStatsTable {
  id: StorageId,
  table: HashMap<BuildingLevel, StorageStats>,
}

impl StorageStatsTable {
  pub(crate) fn new(storage: &dyn Storage) -> Self {
    let max_level = *storage.max_level();
    let mut table = HashMap::with_capacity((max_level).into());

    let mut capacity = f64::from(storage.min_capacity());
    let capacity_growth = growth()
      .floor(capacity)
      .ceil(storage.max_capacity())
      .max_level(max_level)
      .call();

    for level in 1..=max_level {
      let level = BuildingLevel::new(level);
      table.insert(
        level,
        StorageStats {
          level,
          capacity: StorageCapacity::from(capacity.ceil()),
        },
      );

      debug_assert!(capacity.is_normal());

      capacity += capacity * capacity_growth;
    }

    table.shrink_to_fit();

    Self { id: storage.storage_id(), table }
  }

  #[inline]
  pub fn id(&self) -> StorageId {
    self.id
  }

  #[inline]
  pub fn get(&self, level: BuildingLevel) -> Result<&StorageStats> {
    self
      .table
      .get(&level)
      .ok_or(Error::StorageStatsNotFoundForLevel(self.id, level))
  }
}

/// Storage capacity of a building.
#[derive(
  Clone,
  Copy,
  Debug,
  Deref,
  Default,
  From,
  Into,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Deserialize,
  Serialize,
)]
#[into(u32, f64)]
pub struct StorageCapacity(u32);

impl StorageCapacity {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl PartialEq<u32> for StorageCapacity {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

impl Add for StorageCapacity {
  type Output = StorageCapacity;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl Add<u32> for StorageCapacity {
  type Output = StorageCapacity;

  fn add(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_add(rhs))
  }
}

impl AddAssign for StorageCapacity {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl AddAssign<u32> for StorageCapacity {
  fn add_assign(&mut self, rhs: u32) {
    *self = *self + rhs;
  }
}

impl Sub for StorageCapacity {
  type Output = StorageCapacity;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl Sub<u32> for StorageCapacity {
  type Output = StorageCapacity;

  fn sub(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_sub(rhs))
  }
}

impl SubAssign for StorageCapacity {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl SubAssign<u32> for StorageCapacity {
  fn sub_assign(&mut self, rhs: u32) {
    *self = *self - rhs;
  }
}

impl From<f64> for StorageCapacity {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallStorageCapacity {
  pub silo: StorageCapacity,
  pub warehouse: StorageCapacity,
}

impl Add for OverallStorageCapacity {
  type Output = OverallStorageCapacity;

  fn add(mut self, rhs: Self) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign for OverallStorageCapacity {
  fn add_assign(&mut self, rhs: Self) {
    self.silo += rhs.silo;
    self.warehouse += rhs.warehouse;
  }
}

#[derive(Clone, Copy, Debug, Default, Deref, From, Into)]
pub struct StorageCapacityWeight(f64);

#[derive(Clone, Debug)]
pub struct OverallStorageCapacityWeight {
  pub coord: Coord,
  pub silo: StorageCapacityWeight,
  pub warehouse: StorageCapacityWeight,
}

impl OverallStorageCapacityWeight {
  pub fn new(coord: Coord) -> Self {
    Self {
      coord,
      silo: StorageCapacityWeight::default(),
      warehouse: StorageCapacityWeight::default(),
    }
  }
}
