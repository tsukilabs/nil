// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod resources;

use crate::behavior::r#impl::build::{BUILD_TEMPLATE, BuildStep};
use crate::cheat::Cheat;
use crate::city::City;
use crate::city::stability::Stability;
use crate::continent::coord::Coord;
use crate::continent::field::Field;
use crate::continent::index::ContinentKey;
use crate::error::{Error, Result};
use crate::ethic::Ethics;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::building::BuildingId;
use crate::infrastructure::building::r#impl::academy::recruit_queue::AcademyRecruitQueue;
use crate::infrastructure::building::r#impl::prefecture::build_queue::PrefectureBuildQueue;
use crate::infrastructure::building::r#impl::stable::recruit_queue::StableRecruitQueue;
use crate::infrastructure::building::level::BuildingLevel;
use crate::infrastructure::queue::InfrastructureQueue;
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::military::army::Army;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::maneuver::Maneuver;
use crate::npc::bot::BotId;
use crate::player::{Player, PlayerId};
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use nil_util::ops::TryExt;
use std::num::NonZeroU8;
use strum::IntoEnumIterator;
use tap::Pipe;

#[doc(hidden)]
#[macro_export]
macro_rules! bail_if_cheats_are_not_allowed {
  ($world:expr) => {
    if !$world.config.are_cheats_allowed() {
      use $crate::error::Error;
      return Err(Error::CheatingNotAllowed);
    }
  };
}

impl Cheat for World {
  fn get_academy_recruit_queue(&self, key: impl ContinentKey) -> Result<AcademyRecruitQueue> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city(key)?
      .infrastructure()
      .academy()
      .recruit_queue()
      .clone()
      .pipe(Ok)
  }

  fn get_academy_recruit_queues(
    &self,
    coords: &[Coord],
    filter_empty: bool,
  ) -> Result<Vec<(Coord, AcademyRecruitQueue)>> {
    bail_if_cheats_are_not_allowed!(self);
    coords
      .iter()
      .copied()
      .map(|coord| Ok((coord, cheat::get_academy_recruit_queue(self, coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  fn get_all_academy_recruit_queues(
    &self,
    filter_empty: bool,
  ) -> Result<Vec<(Coord, AcademyRecruitQueue)>> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .continent
      .city_coords()
      .map(|coord| Ok((coord, cheat::get_academy_recruit_queue(self, coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  fn get_all_prefecture_build_queues(
    &self,
    filter_empty: bool,
  ) -> Result<Vec<(Coord, PrefectureBuildQueue)>> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .continent
      .city_coords()
      .map(|coord| Ok((coord, cheat::get_prefecture_build_queue(self, coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  fn get_all_stable_recruit_queues(
    &self,
    filter_empty: bool,
  ) -> Result<Vec<(Coord, StableRecruitQueue)>> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .continent
      .city_coords()
      .map(|coord| Ok((coord, cheat::get_stable_recruit_queue(self, coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  fn get_build_steps(&self, key: impl ContinentKey) -> Result<Vec<BuildStep>> {
    bail_if_cheats_are_not_allowed!(self);
    let infrastructure = self.infrastructure(key)?;
    BUILD_TEMPLATE
      .iter()
      .filter(|step| !step.is_done(infrastructure))
      .cloned()
      .collect_vec()
      .pipe(Ok)
  }

  fn get_city(&self, key: impl ContinentKey) -> Result<&City> {
    bail_if_cheats_are_not_allowed!(self);
    self.continent().city(key)
  }

  fn get_ethics(&self, ruler: &Ruler) -> Result<Option<Ethics>> {
    bail_if_cheats_are_not_allowed!(self);
    self.get_ethics(ruler)
  }

  fn get_idle_armies_at(&self, key: impl ContinentKey) -> Result<Vec<Army>> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .military
      .idle_armies_at(key)
      .cloned()
      .collect_vec()
      .pipe(Ok)
  }

  fn get_idle_personnel_at(&self, key: impl ContinentKey) -> Result<ArmyPersonnel> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .military
      .fold_idle_personnel_at(key)
      .pipe(Ok)
  }

  fn get_infrastructure(&self, key: impl ContinentKey) -> Result<Infrastructure> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city(key)?
      .infrastructure()
      .clone()
      .pipe(Ok)
  }

  fn get_maneuvers(&self) -> Result<Vec<Maneuver>> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .military
      .maneuvers()
      .cloned()
      .collect_vec()
      .pipe(Ok)
  }

  fn get_maneuvers_of(&self, ruler: Ruler) -> Result<Vec<Maneuver>> {
    bail_if_cheats_are_not_allowed!(self);

    let mut maneuvers = Vec::new();
    for coord in self.continent.coords_of(ruler) {
      maneuvers.extend(self.military.maneuvers_at(coord));
    }

    maneuvers
      .into_iter()
      .unique_by(|it| it.id())
      .sorted_by_key(|it| it.id())
      .cloned()
      .collect_vec()
      .pipe(Ok)
  }

  fn get_player(&self, id: &PlayerId) -> Result<Player> {
    bail_if_cheats_are_not_allowed!(self);
    self.player(id).cloned()
  }

  fn get_players(&self) -> Result<Vec<Player>> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .players()
      .cloned()
      .collect_vec()
      .pipe(Ok)
  }

  fn get_prefecture_build_queue(&self, key: impl ContinentKey) -> Result<PrefectureBuildQueue> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city(key)?
      .infrastructure()
      .prefecture()
      .build_queue()
      .clone()
      .pipe(Ok)
  }

  fn get_prefecture_build_queues(
    &self,
    coords: &[Coord],
    filter_empty: bool,
  ) -> Result<Vec<(Coord, PrefectureBuildQueue)>> {
    bail_if_cheats_are_not_allowed!(self);
    coords
      .iter()
      .copied()
      .map(|coord| Ok((coord, cheat::get_prefecture_build_queue(self, coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  fn get_stable_recruit_queue(&self, key: impl ContinentKey) -> Result<StableRecruitQueue> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .city(key)?
      .infrastructure()
      .stable()
      .recruit_queue()
      .clone()
      .pipe(Ok)
  }

  fn get_stable_recruit_queues(
    &self,
    coords: &[Coord],
    filter_empty: bool,
  ) -> Result<Vec<(Coord, StableRecruitQueue)>> {
    bail_if_cheats_are_not_allowed!(self);
    coords
      .iter()
      .copied()
      .map(|coord| Ok((coord, cheat::get_stable_recruit_queue(self, coord)?)))
      .filter_ok(|(_, queue)| !filter_empty || !queue.is_empty())
      .try_collect()
  }

  fn get_storage_capacity(&self, ruler: Ruler) -> Result<OverallStorageCapacity> {
    bail_if_cheats_are_not_allowed!(self);
    self.get_storage_capacity(ruler)
  }

  fn set_bot_ethics(&mut self, id: &BotId, ethics: Ethics) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);
    *self.bot_mut(id)?.ethics_mut() = ethics;
    Ok(())
  }

  fn set_building_level(
    &mut self,
    key: impl ContinentKey,
    id: BuildingId,
    level: BuildingLevel,
  ) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);
    let coord = key.into_coord(self.continent.size())?;
    self.set_building_level(coord, id, level)?;
    self.emit_city(coord)?;
    Ok(())
  }

  fn set_max_infrastructure(&mut self, key: impl ContinentKey) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);

    let coord = key.into_coord(self.continent.size())?;
    let infrastructure = self.city_mut(coord)?.infrastructure_mut();

    for id in BuildingId::iter() {
      let building = infrastructure.building_mut(id);
      building.set_level(building.max_level());
    }

    self.emit_city(coord)?;

    Ok(())
  }

  fn set_stability(&mut self, key: impl ContinentKey, stability: Stability) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);

    let coord = key.into_coord(self.continent.size())?;
    let city = self.city_mut(coord)?;
    *city.stability_mut() = stability;

    self.emit_city(coord)?;

    Ok(())
  }

  fn cheat_skip_round(&mut self, amount: NonZeroU8) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);

    if !self.round.is_idle() {
      let amount = amount.get();
      for i in 1..=amount {
        self.dangerously_end_round(i == amount)?;
      }
    }

    Ok(())
  }

  fn spawn_bot(&mut self, name: impl Into<BotId>, infrastructure: Infrastructure) -> Result<BotId> {
    bail_if_cheats_are_not_allowed!(self);
    self.spawn_bot(name, infrastructure)
  }

  fn spawn_city(&mut self, ruler: &Ruler, key: impl ContinentKey) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);

    let coord = key.into_coord(self.continent.size())?;
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

  fn spawn_personnel(
    &mut self,
    key: impl ContinentKey,
    personnel: ArmyPersonnel,
    ruler: Option<Ruler>,
  ) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);

    let coord = key.into_coord(self.continent.size())?;
    let ruler = ruler.unwrap_or_try_else(|| {
      let city = self.city(coord)?;
      Ok(city.owner().clone())
    })?;

    let player = ruler.player().cloned();
    self.military.spawn(coord, ruler, personnel);

    if let Some(player) = player {
      self.emit_military(player)?;
    }

    Ok(())
  }
}
