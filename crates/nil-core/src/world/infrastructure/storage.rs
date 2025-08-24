// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::infrastructure::building::StorageId;
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn get_storage_capacity<R>(&self, owner: R) -> Result<OverallStorageCapacity>
  where
    R: Into<Ruler>,
  {
    let silo_stats = self
      .stats
      .infrastructure
      .storage(StorageId::Silo)?;

    let warehouse_stats = self
      .stats
      .infrastructure
      .storage(StorageId::Warehouse)?;

    self
      .continent
      .cities_of(owner)
      .try_fold(OverallStorageCapacity::default(), |mut acc, city| {
        let infra = city.infrastructure();
        acc.silo += infra
          .storage(StorageId::Silo)
          .capacity(silo_stats)?;

        acc.warehouse += infra
          .storage(StorageId::Warehouse)
          .capacity(warehouse_stats)?;

        Ok(acc)
      })
  }
}
