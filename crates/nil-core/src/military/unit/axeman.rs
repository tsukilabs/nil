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
pub struct Axeman;

impl Axeman {
  pub const ID: UnitId = UnitId::Axeman;
  pub const KIND: UnitKind = UnitKind::Infantry;

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(Power::new(40))
    .infantry_defense(Power::new(10))
    .cavalry_defense(Power::new(5))
    .ranged_defense(Power::new(10))
    .ranged_debuff(RangedDebuff::MIN)
    .speed(Speed::new(18.0))
    .haul(Haul::new(10))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(10),
    cost: Cost::new(1300),
    wood_ratio: ResourceRatio::new(0.45),
    stone_ratio: ResourceRatio::new(0.25),
    iron_ratio: ResourceRatio::new(0.3),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(1),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .academy(BuildingLevel::new(3))
      .build();
}

check_total_resource_ratio!(
  Axeman::CHUNK.wood_ratio,
  Axeman::CHUNK.stone_ratio,
  Axeman::CHUNK.iron_ratio
);
