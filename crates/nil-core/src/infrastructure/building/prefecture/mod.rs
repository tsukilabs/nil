// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

mod build_catalog;
mod build_queue;

use crate::check_total_resource_ratio;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::queue::InfrastructureQueue;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::Score;
use crate::resources::{Cost, MaintenanceRatio, ResourceRatio, Workforce};
use nil_core_macros::Building;
use serde::{Deserialize, Serialize};
use std::ops::Not;

pub use build_catalog::{
  PrefectureBuildCatalog,
  PrefectureBuildCatalogEntry,
  PrefectureBuildCatalogRecipe,
};
pub use build_queue::{
  PrefectureBuildOrder,
  PrefectureBuildOrderId,
  PrefectureBuildOrderKind,
  PrefectureBuildOrderRequest,
  PrefectureBuildOrderState,
  PrefectureBuildQueue,
};

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

  pub const MIN_COST: Cost = Cost::new(1000);
  pub const MAX_COST: Cost = Cost::new(150_000);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.5);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.2);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const MIN_WORKFORCE: Workforce = Workforce::new(1);
  pub const MAX_WORKFORCE: Workforce = Workforce::new(50);

  pub const MIN_SCORE: Score = Score::new(10);
  pub const MAX_SCORE: Score = Score::new(1978);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::none();

  pub fn build_queue(&self) -> &PrefectureBuildQueue {
    &self.build_queue
  }

  pub(crate) fn build_queue_mut(&mut self) -> &mut PrefectureBuildQueue {
    &mut self.build_queue
  }

  #[must_use]
  pub(crate) fn process_queue(&mut self) -> Option<Vec<PrefectureBuildOrder>> {
    if self.enabled {
      let orders = self.build_queue.process(self.workforce());
      orders.is_empty().not().then_some(orders)
    } else {
      None
    }
  }

  #[inline]
  pub fn workforce(&self) -> Workforce {
    self.level.into()
  }

  pub fn resolve_level(&self, building: BuildingId, current_level: BuildingLevel) -> BuildingLevel {
    self
      .build_queue
      .resolve_level(building, current_level)
  }

  pub fn turns_in_queue(&self) -> f64 {
    let turn = self.workforce();
    let queue = self.build_queue.sum_workforce();
    if queue > 0 { f64::from(turn) / f64::from(queue) } else { 0.0 }
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
