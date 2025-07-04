// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::building::{Building, BuildingId, BuildingLevel};
use crate::error::{Error, Result};
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroU32;
use strum::{Display, EnumIter};

/// Um edifício que armazena recursos.
pub trait Storage: Building {
  fn storage_id(&self) -> StorageId;
  /// Capacidade máxima de armazenamento em seu nível máximo.
  fn capacity(&self) -> StorageCapacity;
  /// Taxa de crescimento da capacidade de armazenamento.
  fn capacity_growth(&self) -> StorageCapacityGrowth;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Display, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub enum StorageId {
  Silo,
  Warehouse,
}

impl From<StorageId> for BuildingId {
  fn from(value: StorageId) -> Self {
    match value {
      StorageId::Silo => BuildingId::Silo,
      StorageId::Warehouse => BuildingId::Warehouse,
    }
  }
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
    let mut table = HashMap::with_capacity((max_level + 1).into());

    let mut capacity = f64::from(storage.capacity());
    let capacity_growth = *storage.capacity_growth();

    for level in (0..=max_level).rev() {
      let level = BuildingLevel::new(level);
      table.insert(
        level,
        StorageStats {
          level,
          capacity: StorageCapacity::from(capacity.ceil()),
        },
      );

      debug_assert!(capacity.is_finite());
      debug_assert!(capacity >= 0.0);

      capacity -= capacity * capacity_growth;
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

/// Capacidade de armazenamento do edifício.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct StorageCapacity(NonZeroU32);

impl StorageCapacity {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(if value > 0 {
      unsafe { NonZeroU32::new_unchecked(value) }
    } else {
      NonZeroU32::MIN
    })
  }
}

impl From<StorageCapacity> for f64 {
  fn from(value: StorageCapacity) -> Self {
    f64::from(value.0.get())
  }
}

impl From<f64> for StorageCapacity {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct StorageCapacityGrowth(f64);

impl StorageCapacityGrowth {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(0.0, 1.0))
  }
}
