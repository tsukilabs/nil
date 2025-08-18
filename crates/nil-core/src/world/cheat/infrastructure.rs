// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::continent::Coord;
use crate::error::Result;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::world::World;
use strum::IntoEnumIterator;

impl World {
  pub fn cheat_set_max_infrastructure(&mut self, coord: Coord) -> Result<()> {
    bail_cheat_not_allowed!(self);
    let infra = self.city_mut(coord)?.infrastructure_mut();
    for id in BuildingId::iter() {
      let building = infra.building_mut(id);
      building.set_level(building.max_level());
    }

    self.emit_city_updated(coord);

    Ok(())
  }

  pub fn cheat_set_building_level(
    &mut self,
    coord: Coord,
    id: BuildingId,
    level: BuildingLevel,
  ) -> Result<()> {
    bail_cheat_not_allowed!(self);
    self
      .city_mut(coord)?
      .infrastructure_mut()
      .building_mut(id)
      .set_level(level);

    self.emit_city_updated(coord);

    Ok(())
  }
}
