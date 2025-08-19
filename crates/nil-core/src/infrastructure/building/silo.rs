// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{BuildingId, BuildingLevel, StorageId};
use crate::check_total_resource_ratio;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::infrastructure::storage::StorageCapacity;
use crate::ranking::Score;
use crate::resources::{Cost, MaintenanceRatio, ResourceRatio, Workforce};
use nil_core_macros::{Building, Storage};
use serde::{Deserialize, Serialize};

#[derive(Building, Storage, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Silo {
  level: BuildingLevel,
  enabled: bool,
}

impl Silo {
  pub const ID: BuildingId = BuildingId::Silo;
  pub const STORAGE_ID: StorageId = StorageId::Silo;

  pub const MIN_LEVEL: BuildingLevel = BuildingLevel::ZERO;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);

  pub const MIN_COST: Cost = Cost::new(500);
  pub const MAX_COST: Cost = Cost::new(100_000);

  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.0025);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.45);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.25);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.3);

  pub const MIN_WORKFORCE: Workforce = Workforce::new(2);
  pub const MAX_WORKFORCE: Workforce = Workforce::new(250);

  pub const MIN_CAPACITY: StorageCapacity = StorageCapacity::new(1_000);
  pub const MAX_CAPACITY: StorageCapacity = StorageCapacity::new(250_000);

  pub const MIN_SCORE: Score = Score::new(6);
  pub const MAX_SCORE: Score = Score::new(1187);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::none();
}

impl Default for Silo {
  fn default() -> Self {
    Self {
      level: BuildingLevel::new(1),
      enabled: true,
    }
  }
}

check_total_resource_ratio!(Silo::WOOD_RATIO, Silo::STONE_RATIO, Silo::IRON_RATIO);
