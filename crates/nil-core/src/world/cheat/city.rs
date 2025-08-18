// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::city::Stability;
use crate::continent::Coord;
use crate::error::Result;
use crate::world::World;

impl World {
  pub fn cheat_set_stability(&mut self, coord: Coord, stability: Stability) -> Result<()> {
    bail_cheat_not_allowed!(self);
    let city = self.city_mut(coord)?;
    *city.stability_mut() = stability;
    self.emit_city_updated(coord);
    Ok(())
  }
}
