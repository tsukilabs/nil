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
pub struct Swordsman;

impl Swordsman {
  pub const ID: UnitId = UnitId::Swordsman;
  pub const KIND: UnitKind = UnitKind::Infantry;

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(Power::new(25))
    .infantry_defense(Power::new(50))
    .cavalry_defense(Power::new(15))
    .ranged_defense(Power::new(40))
    .ranged_debuff(RangedDebuff::MIN)
    .speed(Speed::new(22.0))
    .haul(Haul::new(15))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(10),
    cost: Cost::new(1300),
    wood_ratio: ResourceRatio::new(0.25),
    stone_ratio: ResourceRatio::new(0.25),
    iron_ratio: ResourceRatio::new(0.5),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(1),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .academy(BuildingLevel::new(1))
      .build();
}

check_total_resource_ratio!(
  Swordsman::CHUNK.wood_ratio,
  Swordsman::CHUNK.stone_ratio,
  Swordsman::CHUNK.iron_ratio
);
