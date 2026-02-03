// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::stats::prelude::*;
use super::{UnitChunk, UnitChunkSize, UnitId, UnitKind};
use crate::check_total_resource_ratio;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::Score;
use crate::resources::prelude::*;
use nil_core_macros::Unit;

#[derive(Unit, Clone, Debug)]
pub struct Ram;

impl Ram {
  pub const ID: UnitId = UnitId::Ram;
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const BUILDING: BuildingId = BuildingId::Workshop;

  pub const SCORE: Score = Score::new(5);

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(Power::new(2))
    .infantry_defense(Power::new(20))
    .cavalry_defense(Power::new(50))
    .ranged_defense(Power::new(20))
    .ranged_debuff(RangedDebuff::MIN)
    .speed(Speed::new(0.5))
    .haul(Haul::new(0))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(3),
    cost: Cost::new(2100),
    wood_ratio: ResourceRatio::new(0.4),
    stone_ratio: ResourceRatio::new(0.3),
    iron_ratio: ResourceRatio::new(0.3),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(8),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .workshop(BuildingLevel::new(1))
      .build();
}

check_total_resource_ratio!(
  Ram::CHUNK.wood_ratio,
  Ram::CHUNK.stone_ratio,
  Ram::CHUNK.iron_ratio
);
