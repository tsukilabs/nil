// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod building;
mod storage;

use crate::city::City;
use crate::continent::{ContinentKey, Coord};
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::building::BuildingId;
use crate::world::World;

impl World {
  #[inline]
  pub fn infrastructure(&self, key: impl ContinentKey) -> Result<&Infrastructure> {
    self.city(key).map(City::infrastructure)
  }

  pub fn toggle_building(&mut self, coord: Coord, id: BuildingId, enabled: bool) -> Result<()> {
    self
      .continent
      .city_mut(coord)?
      .infrastructure_mut()
      .building_mut(id)
      .toggle(enabled);

    self.emit_city_updated(coord)?;

    Ok(())
  }
}
