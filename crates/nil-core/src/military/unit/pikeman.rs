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
pub struct Pikeman;

impl Pikeman {
  pub const ID: UnitId = UnitId::Pikeman;
  pub const KIND: UnitKind = UnitKind::Infantry;

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(Power::new(10))
    .infantry_defense(Power::new(15))
    .cavalry_defense(Power::new(45))
    .ranged_defense(Power::new(20))
    .ranged_debuff(RangedDebuff::MIN)
    .speed(Speed::new(18.0))
    .haul(Haul::new(25))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(10),
    cost: Cost::new(900),
    wood_ratio: ResourceRatio::new(0.55),
    stone_ratio: ResourceRatio::new(0.35),
    iron_ratio: ResourceRatio::new(0.10),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(1),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .academy(BuildingLevel::new(1))
      .build();
}

check_total_resource_ratio!(
  Pikeman::CHUNK.wood_ratio,
  Pikeman::CHUNK.stone_ratio,
  Pikeman::CHUNK.iron_ratio
);
