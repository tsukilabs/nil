// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::continent::Coord;
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::building::academy::AcademyRecruitQueue;
use crate::infrastructure::building::prefecture::PrefectureBuildQueue;
use crate::infrastructure::building::stable::StableRecruitQueue;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::ruler::Ruler;
use crate::world::World;
use nil_util::result::WrapOk;
use strum::IntoEnumIterator;

impl World {
  pub fn cheat_get_academy_recruit_queue(&self, coord: Coord) -> Result<AcademyRecruitQueue> {
    bail_cheat_not_allowed!(self);
    self
      .city(coord)?
      .infrastructure()
      .academy()
      .recruit_queue()
      .clone()
      .wrap_ok()
  }

  pub fn cheat_get_infrastructure(&self, coord: Coord) -> Result<Infrastructure> {
    bail_cheat_not_allowed!(self);
    self
      .city(coord)?
      .infrastructure()
      .clone()
      .wrap_ok()
  }

  pub fn cheat_get_prefecture_build_queue(&self, coord: Coord) -> Result<PrefectureBuildQueue> {
    bail_cheat_not_allowed!(self);
    self
      .city(coord)?
      .infrastructure()
      .prefecture()
      .build_queue()
      .clone()
      .wrap_ok()
  }

  pub fn cheat_get_stable_recruit_queue(&self, coord: Coord) -> Result<StableRecruitQueue> {
    bail_cheat_not_allowed!(self);
    self
      .city(coord)?
      .infrastructure()
      .stable()
      .recruit_queue()
      .clone()
      .wrap_ok()
  }

  pub fn cheat_get_storage_capacity(&self, ruler: Ruler) -> Result<OverallStorageCapacity> {
    bail_cheat_not_allowed!(self);
    self.get_storage_capacity(ruler)
  }

  pub fn cheat_set_max_infrastructure(&mut self, coord: Coord) -> Result<()> {
    bail_cheat_not_allowed!(self);

    let infra = self.city_mut(coord)?.infrastructure_mut();
    for id in BuildingId::iter() {
      let building = infra.building_mut(id);
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
    bail_cheat_not_allowed!(self);
    self
      .city_mut(coord)?
      .infrastructure_mut()
      .building_mut(id)
      .set_level(level);

    self.emit_city_updated(coord);

    Ok(())
  }
}
