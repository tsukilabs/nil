// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::continent::coord::Coord;
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::building::BuildingId;
use crate::infrastructure::building::r#impl::academy::recruit_queue::AcademyRecruitQueue;
use crate::infrastructure::building::r#impl::prefecture::build_queue::PrefectureBuildQueue;
use crate::infrastructure::building::r#impl::stable::recruit_queue::StableRecruitQueue;
use crate::infrastructure::building::level::BuildingLevel;
use crate::infrastructure::queue::InfrastructureQueue;
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use strum::IntoEnumIterator;
use tap::Pipe;

pub fn get_academy_recruit_queue(world: &World, coord: Coord) -> Result<AcademyRecruitQueue> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .city(coord)?
    .infrastructure()
    .academy()
    .recruit_queue()
    .clone()
    .pipe(Ok)
}

pub fn get_academy_recruit_queues(
  world: &World,
  coords: &[Coord],
  filter_empty: bool,
) -> Result<Vec<(Coord, AcademyRecruitQueue)>> {
   bail_if_cheats_are_not_allowed!(world);
  coords
    .iter()
    .copied()
    .map(|coord| Ok((coord, get_academy_recruit_queue(world, coord)?)))
    .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
    .try_collect()
}

pub fn get_all_academy_recruit_queues(
  world: &World,
  filter_empty: bool,
) -> Result<Vec<(Coord, AcademyRecruitQueue)>> {
   bail_if_cheats_are_not_allowed!(world);
  world
    .continent
    .city_coords()
    .map(|coord| Ok((coord, get_academy_recruit_queue(world, coord)?)))
    .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
    .try_collect()
}

pub fn get_all_prefecture_build_queues(
  world: &World,
  filter_empty: bool,
) -> Result<Vec<(Coord, PrefectureBuildQueue)>> {
   bail_if_cheats_are_not_allowed!(world);
  world
    .continent
    .city_coords()
    .map(|coord| Ok((coord, get_prefecture_build_queue(world, coord)?)))
    .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
    .try_collect()
}

pub fn get_all_stable_recruit_queues(
  world: &World,
  filter_empty: bool,
) -> Result<Vec<(Coord, StableRecruitQueue)>> {
   bail_if_cheats_are_not_allowed!(world);
  world
    .continent
    .city_coords()
    .map(|coord| Ok((coord, get_stable_recruit_queue(world, coord)?)))
    .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
    .try_collect()
}

pub fn get_infrastructure(world: &World, coord: Coord) -> Result<Infrastructure> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .city(coord)?
    .infrastructure()
    .clone()
    .pipe(Ok)
}

pub fn get_prefecture_build_queue(world: &World, coord: Coord) -> Result<PrefectureBuildQueue> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .city(coord)?
    .infrastructure()
    .prefecture()
    .build_queue()
    .clone()
    .pipe(Ok)
}

pub fn get_prefecture_build_queues(
  world: &World,
  coords: &[Coord],
  filter_empty: bool,
) -> Result<Vec<(Coord, PrefectureBuildQueue)>> {
   bail_if_cheats_are_not_allowed!(world);
  coords
    .iter()
    .copied()
    .map(|coord| Ok((coord, get_prefecture_build_queue(world, coord)?)))
    .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
    .try_collect()
}

pub fn get_stable_recruit_queue(world: &World, coord: Coord) -> Result<StableRecruitQueue> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .city(coord)?
    .infrastructure()
    .stable()
    .recruit_queue()
    .clone()
    .pipe(Ok)
}

pub fn get_stable_recruit_queues(
  world: &World,
  coords: &[Coord],
  filter_empty: bool,
) -> Result<Vec<(Coord, StableRecruitQueue)>> {
   bail_if_cheats_are_not_allowed!(world);
  coords
    .iter()
    .copied()
    .map(|coord| Ok((coord, get_stable_recruit_queue(world, coord)?)))
    .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
    .try_collect()
}

pub fn get_storage_capacity(world: &World, ruler: Ruler) -> Result<OverallStorageCapacity> {
  bail_if_cheats_are_not_allowed!(world);
  world.get_storage_capacity(ruler)
}

pub fn set_building_level(
  world: &mut World,
  coord: Coord,
  id: BuildingId,
  level: BuildingLevel,
) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);
  world.set_building_level(coord, id, level)?;
  world.emit_city(coord)?;
  Ok(())
}

pub fn set_max_infrastructure(world: &mut World, coord: Coord) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);

  let infrastructure = world.city_mut(coord)?.infrastructure_mut();

  for id in BuildingId::iter() {
    let building = infrastructure.building_mut(id);
    building.set_level(building.max_level());
  }

  world.emit_city(coord)?;

  Ok(())
}
