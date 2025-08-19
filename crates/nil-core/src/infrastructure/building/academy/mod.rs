// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod recruit_catalog;
mod recruit_queue;

use super::{BuildingId, BuildingLevel};
use crate::check_total_resource_ratio;
use crate::infrastructure::queue::InfrastructureQueue;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::Score;
use crate::resources::{Cost, MaintenanceRatio, ResourceRatio, Workforce};
use nil_core_macros::Building;
use serde::{Deserialize, Serialize};
use std::ops::Not;

pub use recruit_catalog::{
  AcademyRecruitCatalog,
  AcademyRecruitCatalogEntry,
  AcademyRecruitCatalogRecipe,
};
pub use recruit_queue::{
  AcademyRecruitOrder,
  AcademyRecruitOrderId,
  AcademyRecruitOrderRequest,
  AcademyRecruitOrderState,
  AcademyRecruitQueue,
};

// TODO: The implementation of the recruit queues and catalogs is mostly identical.
// We should probably do something to avoid this unnecessary repetition.

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Academy {
  level: BuildingLevel,
  enabled: bool,
  recruit_queue: AcademyRecruitQueue,
}

impl Academy {
  pub const ID: BuildingId = BuildingId::Academy;

  pub const MIN_LEVEL: BuildingLevel = BuildingLevel::ZERO;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(25);

  pub const MIN_COST: Cost = Cost::new(1_000);
  pub const MAX_COST: Cost = Cost::new(50_000);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.4);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const MIN_WORKFORCE: Workforce = Workforce::new(1);
  pub const MAX_WORKFORCE: Workforce = Workforce::new(100);

  pub const MIN_SCORE: Score = Score::new(16);
  pub const MAX_SCORE: Score = Score::new(1272);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .prefecture(BuildingLevel::new(3))
      .build();

  pub fn recruit_queue(&self) -> &AcademyRecruitQueue {
    &self.recruit_queue
  }

  pub(crate) fn recruit_queue_mut(&mut self) -> &mut AcademyRecruitQueue {
    &mut self.recruit_queue
  }

  #[must_use]
  pub(crate) fn process_queue(&mut self) -> Option<Vec<AcademyRecruitOrder>> {
    if self.enabled {
      let orders = self.recruit_queue.process(self.level.into());
      orders.is_empty().not().then_some(orders)
    } else {
      None
    }
  }
}

impl Default for Academy {
  fn default() -> Self {
    Self {
      level: BuildingLevel::ZERO,
      enabled: true,
      recruit_queue: AcademyRecruitQueue::default(),
    }
  }
}

check_total_resource_ratio!(
  Academy::WOOD_RATIO,
  Academy::STONE_RATIO,
  Academy::IRON_RATIO,
);
