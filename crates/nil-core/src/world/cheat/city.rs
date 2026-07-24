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

pub fn get_city(world: &World, key: impl ContinentKey) -> Result<&City> {
  bail_if_cheats_are_not_allowed!(world);
  world.continent.city(key)
}

pub fn set_stability(world: &mut World, coord: Coord, stability: Stability) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);
  let city = world.city_mut(coord)?;
  *city.stability_mut() = stability;
  world.emit_city(coord)?;
  Ok(())
}

pub fn spawn_city(world: &mut World, ruler: &Ruler, coord: Coord) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);
  let city = City::builder(coord)
    .name(<Ruler as AsRef<str>>::as_ref(ruler))
    .owner(ruler.clone())
    .build();

  let field = world.continent.field_mut(coord)?;
  if field.is_empty() {
    *field = Field::City { city: Box::new(city) };
    world.emit_public_city(coord)?;

    if let Some(player) = ruler.player() {
      world.emit_player(player.clone())?;
    }
  } else {
    return Err(Error::FieldNotEmpty(coord));
  }

  Ok(())
}
