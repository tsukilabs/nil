// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::city::City;
use crate::city::stability::Stability;
use crate::continent::field::Field;
use crate::continent::index::ContinentKey;
use crate::error::{Error, Result};
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;

pub fn fill_world(world: &mut World, ruler: &Ruler) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);

  let size = world.continent.size();
  let coords = world
    .continent
    .enumerate_fields()
    .filter(|(_, field)| field.is_empty())
    .filter_map(|(idx, _)| idx.to_coord(size).ok())
    .collect_vec();

  for coord in coords {
    spawn_city_with_emit(world)
      .ruler(ruler)
      .key(coord)
      .emit(false)
      .call()?;

    world.emit_public_city(coord)?;
  }

  if let Some(player) = ruler.player() {
    world.emit_player(player.clone())?;
  }

  Ok(())
}

pub fn get_city(world: &World, key: impl ContinentKey) -> Result<&City> {
  bail_if_cheats_are_not_allowed!(world);
  world.continent.city(key)
}

pub fn set_stability(
  world: &mut World,
  key: impl ContinentKey,
  stability: Stability,
) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);

  let coord = key.into_coord(world.continent.size())?;
  let city = world.city_mut(coord)?;
  *city.stability_mut() = stability.clamped();

  world.emit_city(coord)?;

  Ok(())
}

pub fn spawn_city(world: &mut World, ruler: &Ruler, key: impl ContinentKey) -> Result<()> {
  spawn_city_with_emit(world)
    .ruler(ruler)
    .key(key)
    .emit(true)
    .call()
}

#[bon::builder]
fn spawn_city_with_emit(
  #[builder(start_fn)] world: &mut World,
  ruler: &Ruler,
  key: impl ContinentKey,
  emit: bool,
) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);

  let coord = key.into_coord(world.continent.size())?;
  let city = City::builder(coord)
    .name(ruler)
    .owner(ruler.clone())
    .build();

  let field = world.continent.field_mut(coord)?;
  if field.is_empty() {
    *field = Field::City { city: Box::new(city) };

    if emit {
      world.emit_public_city(coord)?;
      if let Some(player) = ruler.player() {
        world.emit_player(player.clone())?;
      }
    }
  } else {
    return Err(Error::FieldNotEmpty(coord));
  }

  Ok(())
}
