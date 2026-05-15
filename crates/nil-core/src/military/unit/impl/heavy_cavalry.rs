// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::check_total_resource_ratio;
use crate::infrastructure::building::{BuildingId, BuildingLevel};
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::military::unit::stats::prelude::*;
use crate::military::unit::{UnitChunk, UnitChunkSize, UnitId, UnitKind};
use crate::ranking::score::Score;
use crate::resources::prelude::*;
use nil_core_macros::Unit;

#[derive(Unit, Clone, Debug)]
pub struct HeavyCavalry;

impl HeavyCavalry {
  pub const ID: UnitId = UnitId::HeavyCavalry;
  pub const KIND: UnitKind = UnitKind::Cavalry;
  pub const BUILDING: BuildingId = BuildingId::Stable;

  pub const SCORE: Score = Score::new(6);

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(AttackPower::new(150))
    .defense(
      DefensePower::builder()
        .cavalry(Power::new(80))
        .infantry(Power::new(200))
        .ranged(Power::new(180))
        .build(),
    )
    .ranged_debuff(RangedDebuff::MIN)
    .base_speed(Speed::new(3.5))
    .haul(Haul::new(50))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(5),
    cost: Cost::new(4_750),
    food_ratio: ResourceRatio::new(0.0),
    iron_ratio: ResourceRatio::new(0.65),
    stone_ratio: ResourceRatio::new(0.15),
    wood_ratio: ResourceRatio::new(0.2),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(3),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .stable(BuildingLevel::new(3))
      .build();
}

check_total_resource_ratio!(
  HeavyCavalry::CHUNK.food_ratio,
  HeavyCavalry::CHUNK.iron_ratio,
  HeavyCavalry::CHUNK.stone_ratio,
  HeavyCavalry::CHUNK.wood_ratio
);
