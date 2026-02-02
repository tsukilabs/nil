// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::continent::ContinentKey;
use crate::error::Result;
use crate::resources::maintenance::MaintenanceBalance;
use crate::resources::prelude::Maintenance;
use crate::resources::{Food, Resources};
use crate::ruler::Ruler;
use crate::world::World;
use nil_num::ops::MulCeil;

impl World {
  pub(crate) fn get_maintenance<R>(&self, ruler: R) -> Result<Maintenance>
  where
    R: Into<Ruler>,
  {
    let ruler: Ruler = ruler.into();
    let stats = self.stats().infrastructure();
    let mut maintenance = self.military().maintenance_of(ruler.clone());

    for city in self.continent().cities_of(ruler) {
      maintenance += city.maintenance(&stats)?;
    }

    Ok(maintenance)
  }

  pub(crate) fn get_maintenance_balance<R>(&self, ruler: R) -> Result<MaintenanceBalance>
  where
    R: Into<Ruler>,
  {
    let ruler: Ruler = ruler.into();
    let stats = self.stats().infrastructure();

    let mut production = Food::new(0);
    let mut maintenance = self.military().maintenance_of(ruler.clone());

    for city in self.continent().cities_of(ruler) {
      maintenance += city.maintenance(&stats)?;
      production += city.round_production(&stats)?.food;
    }

    Ok(MaintenanceBalance { maintenance, production })
  }

  pub fn get_weighted_resources<K>(&self, key: K) -> Result<Resources>
  where
    K: ContinentKey,
  {
    let city = key
      .into_coord(self.continent.size())
      .and_then(|coord| self.city(coord))?;

    let capacity_weight = self.get_storage_capacity_weight(city.coord())?;
    let mut resources = self.ruler(city.owner())?.resources().clone();

    macro_rules! apply {
      ($res:ident, $storage:ident) => {
        let weight = f64::from(capacity_weight.$storage);
        resources.$res = resources.$res.mul_ceil(weight).into();
      };
    }

    apply!(food, silo);
    apply!(iron, warehouse);
    apply!(stone, warehouse);
    apply!(wood, warehouse);

    Ok(resources)
  }

  pub(crate) fn take_resources_of<R>(&mut self, ruler: R, resources: &mut Resources) -> Result<()>
  where
    R: Into<Ruler>,
  {
    let mut ruler = self.ruler_mut(&ruler.into())?;
    let current_resources = ruler.take_resources();
    if let Some(res) = current_resources.checked_sub(resources) {
      *ruler.resources_mut() = res;
    } else {
      *resources = current_resources;
    }

    Ok(())
  }
}
