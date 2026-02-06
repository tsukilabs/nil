// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::continent::Coord;
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::building::academy::recruit_queue::AcademyRecruitQueue;
use crate::infrastructure::building::prefecture::build_queue::PrefectureBuildQueue;
use crate::infrastructure::building::stable::recruit_queue::StableRecruitQueue;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::queue::InfrastructureQueue;
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use strum::IntoEnumIterator;
use tap::Pipe;

impl World {
  pub fn cheat_get_academy_recruit_queue(&self, coord: Coord) -> Result<AcademyRecruitQueue> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city(coord)?
      .infrastructure()
      .academy()
      .recruit_queue()
      .clone()
      .pipe(Ok)
  }

  pub fn cheat_get_academy_recruit_queues(
    &self,
    coords: &[Coord],
    filter_empty: bool,
  ) -> Result<Vec<(Coord, AcademyRecruitQueue)>> {
    coords
      .iter()
      .copied()
      .map(|coord| Ok((coord, self.cheat_get_academy_recruit_queue(coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  pub fn cheat_get_all_academy_recruit_queues(
    &self,
    filter_empty: bool,
  ) -> Result<Vec<(Coord, AcademyRecruitQueue)>> {
    self
      .continent
      .city_coords()
      .map(|coord| Ok((coord, self.cheat_get_academy_recruit_queue(coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  pub fn cheat_get_all_prefecture_build_queues(
    &self,
    filter_empty: bool,
  ) -> Result<Vec<(Coord, PrefectureBuildQueue)>> {
    self
      .continent
      .city_coords()
      .map(|coord| Ok((coord, self.cheat_get_prefecture_build_queue(coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  pub fn cheat_get_all_stable_recruit_queues(
    &self,
    filter_empty: bool,
  ) -> Result<Vec<(Coord, StableRecruitQueue)>> {
    self
      .continent
      .city_coords()
      .map(|coord| Ok((coord, self.cheat_get_stable_recruit_queue(coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  pub fn cheat_get_infrastructure(&self, coord: Coord) -> Result<Infrastructure> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city(coord)?
      .infrastructure()
      .clone()
      .pipe(Ok)
  }

  pub fn cheat_get_prefecture_build_queue(&self, coord: Coord) -> Result<PrefectureBuildQueue> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city(coord)?
      .infrastructure()
      .prefecture()
      .build_queue()
      .clone()
      .pipe(Ok)
  }

  pub fn cheat_get_prefecture_build_queues(
    &self,
    coords: &[Coord],
    filter_empty: bool,
  ) -> Result<Vec<(Coord, PrefectureBuildQueue)>> {
    coords
      .iter()
      .copied()
      .map(|coord| Ok((coord, self.cheat_get_prefecture_build_queue(coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  pub fn cheat_get_stable_recruit_queue(&self, coord: Coord) -> Result<StableRecruitQueue> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city(coord)?
      .infrastructure()
      .stable()
      .recruit_queue()
      .clone()
      .pipe(Ok)
  }

  pub fn cheat_get_stable_recruit_queues(
    &self,
    coords: &[Coord],
    filter_empty: bool,
  ) -> Result<Vec<(Coord, StableRecruitQueue)>> {
    coords
      .iter()
      .copied()
      .map(|coord| Ok((coord, self.cheat_get_stable_recruit_queue(coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  pub fn cheat_get_storage_capacity(&self, ruler: Ruler) -> Result<OverallStorageCapacity> {
    bail_if_cheats_are_not_allowed!(self);
    self.get_storage_capacity(ruler)
  }

  pub fn cheat_set_max_infrastructure(&mut self, coord: Coord) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);

    let infrastructure = self.city_mut(coord)?.infrastructure_mut();

    for id in BuildingId::iter() {
      let building = infrastructure.building_mut(id);
      building.set_level(building.max_level());
    }

    self.emit_city_updated(coord);

    Ok(())
  }

  pub fn cheat_set_building_level(
    &mut self,
    coord: Coord,
    id: BuildingId,
    level: BuildingLevel,
  ) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city_mut(coord)?
      .infrastructure_mut()
      .building_mut(id)
      .set_level(level);

    self.emit_city_updated(coord);

    Ok(())
  }
}
