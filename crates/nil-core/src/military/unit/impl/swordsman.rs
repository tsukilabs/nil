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
pub struct Swordsman;

impl Swordsman {
  pub const ID: UnitId = UnitId::Swordsman;
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const BUILDING: BuildingId = BuildingId::Academy;

  pub const SCORE: Score = Score::new(1);

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(AttackPower::new(25))
    .defense(
      DefensePower::builder()
        .cavalry(Power::new(15))
        .infantry(Power::new(50))
        .ranged(Power::new(40))
        .build(),
    )
    .ranged_debuff(RangedDebuff::MIN)
    .base_speed(Speed::new(2.0))
    .haul(Haul::new(15))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(10),
    cost: Cost::new(1_300),
    food_ratio: ResourceRatio::new(0.0),
    iron_ratio: ResourceRatio::new(0.5),
    stone_ratio: ResourceRatio::new(0.25),
    wood_ratio: ResourceRatio::new(0.25),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(1),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .academy(BuildingLevel::new(1))
      .build();
}

check_total_resource_ratio!(
  Swordsman::CHUNK.food_ratio,
  Swordsman::CHUNK.iron_ratio,
  Swordsman::CHUNK.stone_ratio,
  Swordsman::CHUNK.wood_ratio
);
