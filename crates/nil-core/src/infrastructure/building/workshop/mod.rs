// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod recruit_catalog;
pub mod recruit_queue;

use crate::check_total_resource_ratio;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::queue::InfrastructureQueue;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::Score;
use crate::resources::cost::{Cost, ResourceRatio};
use crate::resources::maintenance::MaintenanceRatio;
use crate::resources::workforce::Workforce;
use nil_core_macros::Building;
use recruit_queue::{WorkshopRecruitOrder, WorkshopRecruitQueue};
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Workshop {
  level: BuildingLevel,
  enabled: bool,
  recruit_queue: WorkshopRecruitQueue,
}

impl Workshop {
  pub const ID: BuildingId = BuildingId::Workshop;

  pub const MIN_LEVEL: BuildingLevel = BuildingLevel::ZERO;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(15);

  pub const MIN_COST: Cost = Cost::new(800);
  pub const MAX_COST: Cost = Cost::new(25_000);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.4);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const MIN_WORKFORCE: Workforce = Workforce::new(5);
  pub const MAX_WORKFORCE: Workforce = Workforce::new(80);

  pub const MIN_SCORE: Score = Score::new(24);
  pub const MAX_SCORE: Score = Score::new(308);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .prefecture(BuildingLevel::new(3))
      .academy(BuildingLevel::new(1))
      .build();

  pub fn recruit_queue(&self) -> &WorkshopRecruitQueue {
    &self.recruit_queue
  }

  pub(crate) fn recruit_queue_mut(&mut self) -> &mut WorkshopRecruitQueue {
    &mut self.recruit_queue
  }

  #[must_use]
  pub(crate) fn process_queue(&mut self) -> Option<Vec<WorkshopRecruitOrder>> {
    if self.enabled {
      let orders = self.recruit_queue.process(self.level.into());
      (!orders.is_empty()).then_some(orders)
    } else {
      None
    }
  }

  #[inline]
  pub fn workforce(&self) -> Workforce {
    Workforce::from(self.level)
  }

  pub fn turns_in_recruit_queue(&self) -> Option<f64> {
    if self.level > 0u8 {
      let turn = self.workforce();
      let in_queue = self.recruit_queue.sum_pending_workforce();
      Some(f64::from(in_queue) / f64::from(turn))
    } else {
      None
    }
  }
}

impl Default for Workshop {
  fn default() -> Self {
    Self {
      level: BuildingLevel::ZERO,
      enabled: true,
      recruit_queue: WorkshopRecruitQueue::default(),
    }
  }
}

check_total_resource_ratio!(
  Workshop::WOOD_RATIO,
  Workshop::STONE_RATIO,
  Workshop::IRON_RATIO
);
