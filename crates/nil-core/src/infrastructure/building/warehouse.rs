// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{BuildingId, BuildingLevel};
use crate::check_total_resource_ratio;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::infrastructure::storage::{StorageCapacity, StorageCapacityGrowth, StorageId};
use crate::resource::{
  BaseCost,
  BaseCostGrowth,
  MaintenanceRatio,
  ResourceRatio,
  Workforce,
  WorkforceGrowth,
};
use nil_core_macros::{Building, Storage};
use serde::{Deserialize, Serialize};

#[derive(Building, Storage, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Warehouse {
  level: BuildingLevel,
  enabled: bool,
}

impl Warehouse {
  pub const ID: BuildingId = BuildingId::Warehouse;
  pub const STORAGE_ID: StorageId = StorageId::Warehouse;

  pub const MIN_LEVEL: BuildingLevel = BuildingLevel::ZERO;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);

  pub const BASE_COST: BaseCost = BaseCost::new(100_000);
  pub const BASE_COST_GROWTH: BaseCostGrowth = BaseCostGrowth::new(0.2);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.4);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.4);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.2);

  pub const WORKFORCE: Workforce = Workforce::new(250);
  pub const WORKFORCE_GROWTH: WorkforceGrowth = WorkforceGrowth::new(0.2);

  pub const CAPACITY: StorageCapacity = StorageCapacity::new(400_000);
  pub const CAPACITY_GROWTH: StorageCapacityGrowth = StorageCapacityGrowth::new(0.2);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::none();
}

impl Default for Warehouse {
  fn default() -> Self {
    Self {
      level: BuildingLevel::new(1),
      enabled: true,
    }
  }
}

check_total_resource_ratio!(
  Warehouse::WOOD_RATIO,
  Warehouse::STONE_RATIO,
  Warehouse::IRON_RATIO,
);
