// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{BuildingId, BuildingLevel};
use crate::check_total_resource_ratio;
use crate::infrastructure::mine::{MineId, MineProduction, MineProductionGrowth};
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::resource::{
  BaseCost,
  BaseCostGrowth,
  MaintenanceRatio,
  ResourceRatio,
  Workforce,
  WorkforceGrowth,
};
use nil_core_macros::{Building, Mine};
use serde::{Deserialize, Serialize};

#[derive(Building, Mine, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Farm {
  level: BuildingLevel,
  enabled: bool,
}

impl Farm {
  pub const ID: BuildingId = BuildingId::Farm;
  pub const MINE_ID: MineId = MineId::Farm;

  pub const MIN_LEVEL: BuildingLevel = BuildingLevel::ZERO;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);

  pub const BASE_COST: BaseCost = BaseCost::new(300_000);
  pub const BASE_COST_GROWTH: BaseCostGrowth = BaseCostGrowth::new(0.18);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.0);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.35);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.5);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.15);

  pub const WORKFORCE: Workforce = Workforce::new(300);
  pub const WORKFORCE_GROWTH: WorkforceGrowth = WorkforceGrowth::new(0.2);

  pub const PRODUCTION: MineProduction = MineProduction::new(3600);
  pub const PRODUCTION_GROWTH: MineProductionGrowth = MineProductionGrowth::new(0.2);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::none();
}

impl Default for Farm {
  fn default() -> Self {
    Self {
      level: BuildingLevel::new(1),
      enabled: true,
    }
  }
}

check_total_resource_ratio!(Farm::WOOD_RATIO, Farm::STONE_RATIO, Farm::IRON_RATIO);
