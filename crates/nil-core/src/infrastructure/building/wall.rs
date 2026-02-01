// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::check_total_resource_ratio;
use crate::error::{Error, Result};
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::Score;
use crate::resources::cost::{ResourceRatio, Cost};
use crate::resources::maintenance::MaintenanceRatio;
use crate::resources::workforce::Workforce;
use derive_more::Deref;
use nil_core_macros::Building;
use nil_num::growth::growth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wall {
  level: BuildingLevel,
  enabled: bool,
}

impl Wall {
  pub const ID: BuildingId = BuildingId::Wall;

  pub const MIN_LEVEL: BuildingLevel = BuildingLevel::ZERO;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(20);

  pub const MIN_COST: Cost = Cost::new(1_200);
  pub const MAX_COST: Cost = Cost::new(50_000);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.5);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.2);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const MIN_WORKFORCE: Workforce = Workforce::new(2);
  pub const MAX_WORKFORCE: Workforce = Workforce::new(200);

  pub const MIN_DEFENSE: WallDefense = WallDefense::new(500);
  pub const MAX_DEFENSE: WallDefense = WallDefense::new(10000);

  pub const MIN_DEFENSE_BONUS: WallDefenseBonus = WallDefenseBonus::new(5.0);
  pub const MAX_DEFENSE_BONUS: WallDefenseBonus = WallDefenseBonus::new(107.0);

  pub const MIN_SCORE: Score = Score::new(8);
  pub const MAX_SCORE: Score = Score::new(256);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .academy(BuildingLevel::new(1))
      .build();
}

impl Default for Wall {
  fn default() -> Self {
    Self {
      level: BuildingLevel::ZERO,
      enabled: true,
    }
  }
}

check_total_resource_ratio!(Wall::WOOD_RATIO, Wall::STONE_RATIO, Wall::IRON_RATIO);

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct WallDefense(u32);

impl WallDefense {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<WallDefense> for f64 {
  fn from(value: WallDefense) -> Self {
    f64::from(value.0)
  }
}

impl From<f64> for WallDefense {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct WallDefenseBonus(f64);

impl WallDefenseBonus {
  pub const MIN: WallDefenseBonus = WallDefenseBonus(0.0);
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.max(Self::MIN.0))
  }
}

impl From<WallDefenseBonus> for f64 {
  fn from(value: WallDefenseBonus) -> Self {
    value.0
  }
}

impl From<f64> for WallDefenseBonus {
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WallStats {
  pub level: BuildingLevel,
  pub defense: WallDefense,
  pub defense_percent: WallDefenseBonus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WallStatsTable(HashMap<BuildingLevel, WallStats>);

impl WallStatsTable {
  pub fn new() -> Self {
    let max_level = Wall::MAX_LEVEL;
    let mut table = HashMap::with_capacity((*max_level).into());

    let mut defense = f64::from(Wall::MIN_DEFENSE);
    let defense_growth = growth()
      .floor(defense)
      .ceil(f64::from(Wall::MAX_DEFENSE))
      .max_level(max_level)
      .call();

    let mut defense_percent = f64::from(Wall::MIN_DEFENSE_BONUS);
    let defense_percent_growth = growth()
      .floor(defense_percent)
      .ceil(f64::from(Wall::MAX_DEFENSE_BONUS))
      .max_level(max_level)
      .call();

    for level in 1..=max_level.0 {
      let level = BuildingLevel::new(level);

      table.insert(
        level,
        WallStats {
          level,
          defense: WallDefense::from(defense.round()),
          defense_percent: WallDefenseBonus::from(defense_percent.round()),
        },
      );

      defense += defense * defense_growth;
      defense_percent += defense_percent * defense_percent_growth;
    }

    table.shrink_to_fit();

    Self(table)
  }

  #[inline]
  pub fn get(&self, level: BuildingLevel) -> Result<&WallStats> {
    self
      .0
      .get(&level)
      .ok_or(Error::WallStatsNotFoundForLevel(level))
  }
}

impl Default for WallStatsTable {
  fn default() -> Self {
    Self::new()
  }
}
