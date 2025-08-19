// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{BuildingId, BuildingLevel, MineId};
use crate::check_total_resource_ratio;
use crate::infrastructure::mine::MineProduction;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::Score;
use crate::resources::{Cost, MaintenanceRatio, ResourceRatio, Workforce};
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

  pub const MIN_COST: Cost = Cost::new(800);
  pub const MAX_COST: Cost = Cost::new(100_000);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.35);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.5);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.15);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.0);

  pub const MIN_WORKFORCE: Workforce = Workforce::new(5);
  pub const MAX_WORKFORCE: Workforce = Workforce::new(300);

  pub const MIN_PRODUCTION: MineProduction = MineProduction::new(30);
  pub const MAX_PRODUCTION: MineProduction = MineProduction::new(2400);

  pub const MIN_SCORE: Score = Score::new(6);
  pub const MAX_SCORE: Score = Score::new(1187);

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
