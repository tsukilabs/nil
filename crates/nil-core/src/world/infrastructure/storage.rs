// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::ContinentKey;
use crate::error::Result;
use crate::infrastructure::storage::{
  OverallStorageCapacity,
  OverallStorageCapacityWeight,
  StorageCapacityWeight,
};
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn get_storage_capacity<R>(&self, ruler: R) -> Result<OverallStorageCapacity>
  where
    R: Into<Ruler>,
  {
    let stats = &self.stats.infrastructure;
    self
      .continent
      .cities_of(ruler)
      .try_fold(OverallStorageCapacity::default(), |mut acc, city| {
        acc += city.storage_capacity(stats)?;
        Ok(acc)
      })
  }

  pub fn get_storage_capacity_weight<K>(&self, key: K) -> Result<OverallStorageCapacityWeight>
  where
    K: ContinentKey,
  {
    let stats = &self.stats.infrastructure;
    let city = key
      .into_coord(self.continent.size())
      .and_then(|coord| self.city(coord))?;

    let capacity = city.storage_capacity(stats)?;
    let total = self.get_storage_capacity(city.owner().clone())?;

    let mut weight = OverallStorageCapacityWeight::new(city.coord());

    macro_rules! set_weight {
      ($($storage:ident),+ $(,)?) => {
        $(
          if *total.$storage > 0u32 {
            let value = f64::from(capacity.$storage) / f64::from(total.$storage);
            weight.$storage = StorageCapacityWeight::from(value);
            debug_assert!(value.is_normal());
          }
        )+
      };
    }

    set_weight!(silo, warehouse);

    Ok(weight)
  }
}
