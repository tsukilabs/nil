// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod building;
mod storage;

use super::World;
use crate::continent::Coord;
use crate::error::Result;
use crate::infrastructure::building::BuildingId;

impl World {
  pub fn toggle_building(&mut self, coord: Coord, id: BuildingId, enabled: bool) -> Result<()> {
    self
      .continent
      .city_mut(coord)?
      .infrastructure_mut()
      .building_mut(id)
      .toggle(enabled);

    self.emit_city_updated(coord);

    Ok(())
  }
}
