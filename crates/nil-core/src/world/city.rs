// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::continent::Coord;
use crate::error::Result;

impl World {
  pub fn rename_city(&mut self, coord: Coord, name: &str) -> Result<()> {
    let city = self.continent.city_mut(coord)?;
    name.clone_into(city.name_mut());

    self.emit_public_city_updated(coord);
    self.emit_city_updated(coord);

    Ok(())
  }
}
