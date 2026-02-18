// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod recruit_catalog;
pub mod recruit_queue;

use crate::check_total_resource_ratio;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::queue::InfrastructureQueue;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::score::Score;
use crate::resources::cost::{Cost, ResourceRatio};
use crate::resources::maintenance::MaintenanceRatio;
use crate::resources::workforce::Workforce;
use nil_core_macros::Building;
use recruit_queue::{StableRecruitOrder, StableRecruitQueue};
use serde::{Deserialize, Serialize};

#[derive(Building, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stable {
  level: BuildingLevel,
  enabled: bool,
  recruit_queue: StableRecruitQueue,
}

impl Stable {
  pub const ID: BuildingId = BuildingId::Stable;

  pub const MIN_LEVEL: BuildingLevel = BuildingLevel::ZERO;
  pub const MAX_LEVEL: BuildingLevel = BuildingLevel::new(20);

  pub const MIN_COST: Cost = Cost::new(1_500);
  pub const MAX_COST: Cost = Cost::new(50_000);

  pub const WOOD_RATIO: ResourceRatio = ResourceRatio::new(0.4);
  pub const STONE_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const IRON_RATIO: ResourceRatio = ResourceRatio::new(0.3);
  pub const MAINTENANCE_RATIO: MaintenanceRatio = MaintenanceRatio::new(0.005);

  pub const MIN_WORKFORCE: Workforce = Workforce::new(1);
  pub const MAX_WORKFORCE: Workforce = Workforce::new(150);

  pub const MIN_SCORE: Score = Score::new(20);
  pub const MAX_SCORE: Score = Score::new(639);

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .prefecture(BuildingLevel::new(3))
      .academy(BuildingLevel::new(1))
      .build();

  pub fn recruit_queue(&self) -> &StableRecruitQueue {
    &self.recruit_queue
  }

  pub(crate) fn recruit_queue_mut(&mut self) -> &mut StableRecruitQueue {
    &mut self.recruit_queue
  }

  #[must_use]
  pub(crate) fn process_queue(&mut self) -> Option<Vec<StableRecruitOrder>> {
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

impl Default for Stable {
  fn default() -> Self {
    Self {
      level: BuildingLevel::ZERO,
      enabled: true,
      recruit_queue: StableRecruitQueue::default(),
    }
  }
}

check_total_resource_ratio!(Stable::WOOD_RATIO, Stable::STONE_RATIO, Stable::IRON_RATIO);
