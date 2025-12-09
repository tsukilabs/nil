// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::continent::ContinentKey;
use crate::error::Result;
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::world::World;
use nil_num::ops::MulCeil;

impl World {
  pub fn get_weighted_resources<K>(&self, key: K) -> Result<Resources>
  where
    K: ContinentKey,
  {
    let city = key
      .into_coord(self.continent.size())
      .and_then(|coord| self.city(coord))?;

    let capacity_weight = self.get_storage_capacity_weight(city.coord())?;
    let mut resources = self
      .ruler(city.owner().clone())?
      .resources()
      .clone();

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
    let mut ruler = self.ruler_mut(ruler.into())?;
    let current_resources = ruler.take_resources();
    if let Some(res) = current_resources.checked_sub(resources) {
      *ruler.resources_mut() = res;
    } else {
      *resources = current_resources;
    }

    Ok(())
  }
}
