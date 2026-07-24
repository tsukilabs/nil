// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::continent::index::ContinentKey;
use crate::error::Result;
use crate::military::army::Army;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::maneuver::Maneuver;
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use nil_util::ops::TryExt;
use tap::Pipe;

pub fn get_idle_armies_at(world: &World, key: impl ContinentKey) -> Result<Vec<Army>> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .military
    .idle_armies_at(key)
    .cloned()
    .collect_vec()
    .pipe(Ok)
}

pub fn get_idle_personnel_at(world: &World, key: impl ContinentKey) -> Result<ArmyPersonnel> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .military
    .fold_idle_personnel_at(key)
    .pipe(Ok)
}

pub fn get_maneuvers(world: &World) -> Result<Vec<Maneuver>> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .military
    .maneuvers()
    .cloned()
    .collect_vec()
    .pipe(Ok)
}

pub fn get_maneuvers_of(world: &World, ruler: impl Into<Ruler>) -> Result<Vec<Maneuver>> {
  bail_if_cheats_are_not_allowed!(world);

  let mut maneuvers = Vec::new();
  for coord in world.continent.coords_of(ruler) {
    maneuvers.extend(world.military.maneuvers_at(coord));
  }

  maneuvers
    .into_iter()
    .unique_by(|it| it.id())
    .sorted_by_key(|it| it.id())
    .cloned()
    .collect_vec()
    .pipe(Ok)
}

pub fn spawn_personnel(
  world: &mut World,
  key: impl ContinentKey,
  personnel: ArmyPersonnel,
  ruler: Option<Ruler>,
) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);

  let coord = key.into_coord(world.continent.size())?;
  let ruler = ruler.unwrap_or_try_else(|| {
    let city = world.city(coord)?;
    Ok(city.owner().clone())
  })?;

  let player = ruler.player().cloned();
  world.military.spawn(coord, ruler, personnel);

  if let Some(player) = player {
    world.emit_military(player)?;
  }

  Ok(())
}
