// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod build_queue;
mod catalog;

use super::{BuildingId, BuildingLevel};
use crate::check_total_resource_ratio;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::resource::{
  BaseCost,
  BaseCostGrowth,
  MaintenanceRatio,
  ResourceRatio,
  Workforce,
  WorkforceGrowth,
};
use nil_core_macros::Building;
use serde::{Deserialize, Serialize};

pub use build_queue::{
  PrefectureBuildOrder,
  PrefectureBuildOrderId,
  PrefectureBuildOrderKind,
  PrefectureBuildOrderOptions,
  PrefectureBuildOrderStatus,
  PrefectureBuildQueue,
};
pub use catalog::{PrefectureCatalog, PrefectureCatalogEntry, PrefectureCatalogRecipe};

/// Centro logístico da aldeia, responsável pela construção de edifícios.
#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Prefecture {
  level: BuildingLevel,
  enabled: bool,
  build_queue: PrefectureBuildQueue,
}

impl Prefecture {
  pub const ID: BuildingId = BuildingId::Prefecture;

  pub const MIN_LEVEL: BuildingLevel = BuildingLevel::new(1);
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(30);

  pub const BASE_COST: BaseCost = BaseCost::new(150_000);
  pub const BASE_COST_GROWTH: BaseCostGrowth = BaseCostGrowth::new(0.2);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.5);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.2);

  pub const WORKFORCE: Workforce = Workforce::new(50);
  pub const WORKFORCE_GROWTH: WorkforceGrowth = WorkforceGrowth::new(0.2);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::none();

  pub(crate) fn build_queue_mut(&mut self) -> &mut PrefectureBuildQueue {
    &mut self.build_queue
  }

  pub(crate) fn process_queue(&mut self) {
    if self.enabled {
      self.build_queue.process(self.level.into());
    }
  }
}

impl Default for Prefecture {
  fn default() -> Self {
    Self {
      level: BuildingLevel::new(1),
      enabled: true,
      build_queue: PrefectureBuildQueue::default(),
    }
  }
}

check_total_resource_ratio!(
  Prefecture::WOOD_RATIO,
  Prefecture::STONE_RATIO,
  Prefecture::IRON_RATIO,
);
