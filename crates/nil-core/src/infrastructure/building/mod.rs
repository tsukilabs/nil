// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod r#impl;
pub mod level;

use crate::error::{Error, Result};
use crate::infrastructure::building::level::BuildingLevel;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::score::Score;
use crate::resources::prelude::*;
use nil_num::growth::growth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{EnumIs, EnumIter};
use subenum::subenum;

pub trait Building: Send + Sync {
  fn id(&self) -> BuildingId;

  /// Checks whether the building is enabled.
  fn is_enabled(&self) -> bool;
  /// Enables or disables the building.
  fn toggle(&mut self, enabled: bool);

  /// Current building level.
  fn level(&self) -> BuildingLevel;
  /// Minimum building level.
  fn min_level(&self) -> BuildingLevel;
  /// Maximum building level.
  fn max_level(&self) -> BuildingLevel;
  /// Sets the building's level while ensuring it remains within the level limit.
  fn set_level(&mut self, level: BuildingLevel);

  /// Sets the building to its **minimum** level.
  fn set_min_level(&mut self) {
    self.set_level(self.min_level());
  }

  /// Sets the building to its **maximum** level.
  fn set_max_level(&mut self) {
    self.set_level(self.max_level());
  }

  /// Increases the building level by one, if possible.
  fn increase_level(&mut self) {
    self.increase_level_by(1);
  }

  /// Increases the level of the building by a certain amount, if possible.
  fn increase_level_by(&mut self, amount: u8);

  /// Decreases the building level by one, if possible.
  fn decrease_level(&mut self) {
    self.decrease_level_by(1);
  }

  /// Decreases the level of the building by a certain amount, if possible.
  fn decrease_level_by(&mut self, amount: u8);

  /// Checks whether the building is at its minimum level.
  fn is_min_level(&self) -> bool {
    self.level() == self.min_level()
  }

  /// Checks whether the building is at its maximum level.
  fn is_max_level(&self) -> bool {
    self.level() >= self.max_level()
  }

  /// Total cost for the **minimum** level of the building.
  fn min_cost(&self) -> Cost;
  /// Total cost for the **maximum** level of the building.
  fn max_cost(&self) -> Cost;

  /// Percentage of the total cost related to food.
  fn food_ratio(&self) -> ResourceRatio;
  /// Percentage of the total cost related to iron.
  fn iron_ratio(&self) -> ResourceRatio;
  /// Percentage of the total cost related to stone.
  fn stone_ratio(&self) -> ResourceRatio;
  /// Percentage of the total cost related to wood.
  fn wood_ratio(&self) -> ResourceRatio;

  /// Building maintenance tax at its current level.
  fn maintenance(&self, stats: &BuildingStatsTable) -> Result<Maintenance>;
  /// Proportion of the base cost used as a maintenance tax.
  fn maintenance_ratio(&self) -> MaintenanceRatio;

  /// Workforce required for the **minimum** level of the building.
  fn min_workforce(&self) -> Workforce;
  /// Workforce required for the **maximum** level of the building.
  fn max_workforce(&self) -> Workforce;

  // Current score.
  fn score(&self, stats: &BuildingStatsTable) -> Result<Score>;
  // Building score at its **minimum** level.
  fn min_score(&self) -> Score;
  // Building score at its **maximum** level.
  fn max_score(&self) -> Score;

  /// Levels required to construct the building.
  fn infrastructure_requirements(&self) -> &InfrastructureRequirements;

  fn is_civil(&self) -> bool {
    self.id().is_civil()
  }

  fn is_military(&self) -> bool {
    self.id().is_military()
  }

  fn is_mine(&self) -> bool {
    self.id().is_mine()
  }

  fn is_storage(&self) -> bool {
    self.id().is_storage()
  }
}

#[subenum(CivilBuildingId, MilitaryBuildingId, MineId, StorageId)]
#[derive(
  Clone, Copy, Debug, strum::Display, EnumIs, EnumIter, PartialEq, Eq, Hash, Deserialize, Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum BuildingId {
  #[subenum(MilitaryBuildingId)]
  Academy,

  #[subenum(CivilBuildingId, MineId)]
  Farm,

  #[subenum(CivilBuildingId, MineId)]
  IronMine,

  #[subenum(CivilBuildingId)]
  Prefecture,

  #[subenum(CivilBuildingId, MineId)]
  Quarry,

  #[subenum(CivilBuildingId, MineId)]
  Sawmill,

  #[subenum(CivilBuildingId, StorageId)]
  Silo,

  #[subenum(MilitaryBuildingId)]
  Stable,

  Wall,

  #[subenum(CivilBuildingId, StorageId)]
  Warehouse,

  #[subenum(MilitaryBuildingId)]
  Workshop,
}

impl BuildingId {
  #[inline]
  pub fn is_civil(self) -> bool {
    CivilBuildingId::try_from(self).is_ok()
  }

  #[inline]
  pub fn is_military(self) -> bool {
    MilitaryBuildingId::try_from(self).is_ok()
  }

  #[inline]
  pub fn is_mine(self) -> bool {
    MineId::try_from(self).is_ok()
  }

  #[inline]
  pub fn is_storage(self) -> bool {
    StorageId::try_from(self).is_ok()
  }
}

/// Information about a building at a given level.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct BuildingStats {
  pub level: BuildingLevel,
  pub cost: Cost,
  pub resources: Resources,
  pub maintenance: Maintenance,
  pub workforce: Workforce,
  pub score: Score,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct BuildingStatsTable {
  id: BuildingId,
  min_level: BuildingLevel,
  max_level: BuildingLevel,
  table: HashMap<BuildingLevel, BuildingStats>,
}

impl BuildingStatsTable {
  pub(crate) fn new(building: &dyn Building) -> Self {
    let min_level = building.min_level();
    let max_level = building.max_level();
    let mut table = HashMap::with_capacity(max_level.into());

    let mut cost = f64::from(building.min_cost());
    let cost_growth = growth()
      .floor(cost)
      .ceil(building.max_cost())
      .max_level(max_level)
      .call();

    let mut workforce = f64::from(building.min_workforce());
    let workforce_growth = growth()
      .floor(workforce)
      .ceil(building.max_workforce())
      .max_level(max_level)
      .call();

    let mut score = f64::from(building.min_score());
    let score_growth = growth()
      .floor(score)
      .ceil(building.max_score())
      .max_level(max_level)
      .call();

    let food_ratio = *building.food_ratio();
    let iron_ratio = *building.iron_ratio();
    let stone_ratio = *building.stone_ratio();
    let wood_ratio = *building.wood_ratio();

    let maintenance_ratio = *building.maintenance_ratio();
    let mut maintenance = cost * maintenance_ratio;

    for level in 1..=u8::from(max_level) {
      let level = BuildingLevel::new(level);
      let resources = Resources {
        food: Food::from((cost * food_ratio).round()),
        iron: Iron::from((cost * iron_ratio).round()),
        stone: Stone::from((cost * stone_ratio).round()),
        wood: Wood::from((cost * wood_ratio).round()),
      };

      table.insert(
        level,
        BuildingStats {
          level,
          cost: Cost::from(cost.round()),
          resources,
          maintenance: Maintenance::from(maintenance.round()),
          workforce: Workforce::from(workforce.round()),
          score: Score::from(score.round()),
        },
      );

      debug_assert!(cost.is_normal());
      debug_assert!(workforce.is_normal());

      debug_assert!(maintenance.is_finite());
      debug_assert!(maintenance >= 0.0);

      debug_assert!(score.is_finite());
      debug_assert!(score >= 0.0);

      cost += cost * cost_growth;
      workforce += workforce * workforce_growth;
      score += score * score_growth;

      maintenance = cost * maintenance_ratio;
    }

    table.shrink_to_fit();

    Self {
      id: building.id(),
      min_level,
      max_level,
      table,
    }
  }

  #[inline]
  pub fn id(&self) -> BuildingId {
    self.id
  }

  #[inline]
  pub fn min_level(&self) -> BuildingLevel {
    self.min_level
  }

  #[inline]
  pub fn max_level(&self) -> BuildingLevel {
    self.max_level
  }

  #[inline]
  pub fn get(&self, level: BuildingLevel) -> Result<&BuildingStats> {
    self
      .table
      .get(&level)
      .ok_or(Error::BuildingStatsNotFoundForLevel(self.id, level))
  }
}
