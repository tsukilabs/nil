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

  pub(crate) fn transpose_resources<A, B>(
    &mut self,
    from: A,
    to: B,
    mut resources: Resources,
  ) -> Result<()>
  where
    A: Into<Ruler>,
    B: Into<Ruler>,
  {
    let mut from = self.ruler_mut(from.into())?;
    let current_resources = from.take_resources();
    if let Some(res) = current_resources.checked_sub(&resources) {
      *from.resources_mut() = res;
    } else {
      resources = current_resources;
    }

    *self.ruler_mut(to.into())?.resources_mut() += resources;

    Ok(())
  }
}
