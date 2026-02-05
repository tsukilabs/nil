// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::continent::Coord;
use crate::error::Result;
use crate::military::army::{Army, ArmyPersonnel};
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use nil_util::ops::TryElse;
use tap::Pipe;

impl World {
  pub fn cheat_get_idle_armies_at(&self, coord: Coord) -> Result<Vec<Army>> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .military
      .idle_armies_at(coord)
      .cloned()
      .collect_vec()
      .pipe(Ok)
  }

  pub fn cheat_get_idle_personnel_at(&self, coord: Coord) -> Result<ArmyPersonnel> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .military
      .fold_idle_personnel_at(coord)
      .pipe(Ok)
  }

  pub fn cheat_spawn_personnel(
    &mut self,
    coord: Coord,
    personnel: ArmyPersonnel,
    ruler: Option<Ruler>,
  ) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);

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
