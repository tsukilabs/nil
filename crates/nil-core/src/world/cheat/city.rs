// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::city::City;
use crate::city::stability::Stability;
use crate::continent::coord::Coord;
use crate::continent::field::Field;
use crate::continent::index::ContinentKey;
use crate::error::{Error, Result};
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn cheat_get_city(&self, key: impl ContinentKey) -> Result<&City> {
    bail_if_cheats_are_not_allowed!(self);
    self.continent.city(key)
  }

  pub fn cheat_set_stability(&mut self, coord: Coord, stability: Stability) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);
    let city = self.city_mut(coord)?;
    *city.stability_mut() = stability;
    self.emit_city(coord)?;
    Ok(())
  }

  pub fn cheat_spawn_city(&mut self, ruler: &Ruler, coord: Coord) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);
    let city = City::builder(coord)
      .name(<Ruler as AsRef<str>>::as_ref(ruler))
      .owner(ruler.clone())
      .build();

    let field = self.continent.field_mut(coord)?;
    if field.is_empty() {
      *field = Field::City { city: Box::new(city) };
      self.emit_public_city(coord)?;

      if let Some(player) = ruler.player() {
        self.emit_player(player.clone())?;
      }
    } else {
      return Err(Error::FieldNotEmpty(coord));
    }

    Ok(())
  }
}
