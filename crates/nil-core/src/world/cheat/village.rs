// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::error::{Error, Result};
use crate::village::{Coord, Stability};
use crate::world::World;

impl World {
  pub fn cheat_set_stability(&mut self, coord: Coord, stability: Stability) -> Result<()> {
    bail_cheat_not_allowed!(self);
    let village = self.village_mut(coord)?;
    *village.stability_mut() = stability;
    self.emit_village_updated(coord);
    Ok(())
  }
}
