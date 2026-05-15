// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::check_total_resource_ratio;
use crate::infrastructure::building::BuildingId;
use crate::infrastructure::building::level::BuildingLevel;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::military::unit::stats::prelude::*;
use crate::military::unit::{UnitChunk, UnitChunkSize, UnitId, UnitKind};
use crate::ranking::score::Score;
use crate::resources::prelude::*;
use nil_core_macros::Unit;

#[derive(Unit, Clone, Debug)]
pub struct Pikeman;

impl Pikeman {
  pub const ID: UnitId = UnitId::Pikeman;
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const BUILDING: BuildingId = BuildingId::Academy;

  pub const SCORE: Score = Score::new(1);

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(AttackPower::new(10))
    .defense(
      DefensePower::builder()
        .cavalry(Power::new(45))
        .infantry(Power::new(15))
        .ranged(Power::new(20))
        .build(),
    )
    .ranged_debuff(RangedDebuff::MIN)
    .base_speed(Speed::new(2.0))
    .haul(Haul::new(25))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(10),
    cost: Cost::new(900),
    food_ratio: ResourceRatio::new(0.0),
    iron_ratio: ResourceRatio::new(0.10),
    stone_ratio: ResourceRatio::new(0.35),
    wood_ratio: ResourceRatio::new(0.55),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(1),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .academy(BuildingLevel::new(1))
      .build();
}

check_total_resource_ratio!(
  Pikeman::CHUNK.food_ratio,
  Pikeman::CHUNK.iron_ratio,
  Pikeman::CHUNK.stone_ratio,
  Pikeman::CHUNK.wood_ratio
);
