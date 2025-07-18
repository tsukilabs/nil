// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{BuildingId, BuildingLevel};
use crate::check_total_resource_ratio;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::resource::{Cost, MaintenanceRatio, ResourceRatio, Workforce};
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

  pub const MIN_DEFENSE_BOOST_PERCENT: WallDefensePercent = WallDefensePercent::new(5.0);
  pub const MAX_DEFENSE_BOOST_PERCENT: WallDefensePercent = WallDefensePercent::new(20.0);

  pub const MIN_DEFENSE: WallDefenseValue = WallDefenseValue::new(2000);
  pub const MAX_DEFENSE: WallDefenseValue = WallDefenseValue::new(20000);

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

/// Proporção entre o custo total e um dado recurso.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct WallDefensePercent(f64);

impl WallDefensePercent {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value)
  }

  #[inline]
  pub const fn as_f64(self) -> f64 {
    self.0
  }
}

impl From<WallDefensePercent> for f64 {
  fn from(value: WallDefensePercent) -> Self {
    value.0
  }
}

impl From<f64> for WallDefensePercent {
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}

/// Custo base de uma entidade, como edifícios ou unidades.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct WallDefenseValue(u32);

impl WallDefenseValue {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<WallDefenseValue> for f64 {
  fn from(value: WallDefenseValue) -> Self {
    f64::from(value.0)
  }
}

impl From<f64> for WallDefenseValue {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WallStats {
  pub level: BuildingLevel,
  pub defense: WallDefenseValue,
  pub defense_percent: WallDefensePercent,
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
      .ceil(Wall::MAX_DEFENSE)
      .max_level(max_level)
      .call();

    let mut defense_percent = f64::from(Wall::MIN_DEFENSE_BOOST_PERCENT);
    let defense_growth_percent = growth()
      .floor(defense_percent)
      .ceil(Wall::MAX_DEFENSE_BOOST_PERCENT)
      .max_level(max_level)
      .call();

    for level in 1..=max_level.0 {
      let level = BuildingLevel::new(level);

      table.insert(
        level,
        WallStats {
          level,
          defense: WallDefenseValue::from(defense.round()),
          defense_percent: WallDefensePercent::from(defense_percent.round()),
        },
      );

      defense += defense * defense_growth;
      defense_percent += defense_percent * defense_growth_percent;
    }

    table.shrink_to_fit();

    Self(table)
  }
}

impl Default for WallStatsTable {
  fn default() -> Self {
    Self::new()
  }
}
