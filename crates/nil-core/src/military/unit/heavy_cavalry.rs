// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::stats::prelude::*;
use super::{UnitChunk, UnitChunkSize, UnitId, UnitKind};
use crate::check_total_resource_ratio;
use crate::infrastructure::building::BuildingLevel;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::resources::prelude::*;
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct HeavyCavalry;

impl HeavyCavalry {
  pub const ID: UnitId = UnitId::HeavyCavalry;
  pub const KIND: UnitKind = UnitKind::Cavalry;

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(Power::new(150))
    .infantry_defense(Power::new(200))
    .cavalry_defense(Power::new(80))
    .ranged_defense(Power::new(180))
    .ranged_debuff(RangedDebuff::MIN)
    .speed(Speed::new(11.0))
    .haul(Haul::new(50))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(5),
    cost: Cost::new(4750),
    wood_ratio: ResourceRatio::new(0.2),
    stone_ratio: ResourceRatio::new(0.15),
    iron_ratio: ResourceRatio::new(0.65),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(3),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .stable(BuildingLevel::new(10))
      .build();
}

check_total_resource_ratio!(
  HeavyCavalry::CHUNK.wood_ratio,
  HeavyCavalry::CHUNK.stone_ratio,
  HeavyCavalry::CHUNK.iron_ratio
);
