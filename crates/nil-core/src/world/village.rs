// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::continent::Coord;
use crate::error::Result;

impl World {
  pub fn rename_village(&mut self, coord: Coord, name: &str) -> Result<()> {
    let village = self.continent.village_mut(coord)?;
    name.clone_into(village.name_mut());

    self.emit_public_village_updated(coord);
    self.emit_village_updated(coord);

    Ok(())
  }
}
