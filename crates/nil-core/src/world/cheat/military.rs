// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::continent::Coord;
use crate::error::Result;
use crate::military::army::ArmyPersonnel;
use crate::ruler::Ruler;
use crate::world::World;
use nil_util::ops::TryElse;

impl World {
  pub fn cheat_spawn_personnel(
    &mut self,
    coord: Coord,
    personnel: ArmyPersonnel,
    ruler: Option<Ruler>,
  ) -> Result<()> {
    bail_cheat_not_allowed!(self);
    let ruler = ruler.unwrap_or_try_else(|| {
      let city = self.city(coord)?;
      Ok(city.owner().clone())
    })?;

    let player = ruler.player().cloned();
    self.military.spawn(coord, ruler, personnel);

    if let Some(player) = player {
      self.emit_military_updated(player);
    }

    Ok(())
  }
}
