// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::stats::prelude::*;
use super::{UnitChunk, UnitChunkSize, UnitId, UnitKind};
use crate::check_total_resource_ratio;
use crate::infrastructure::building::BuildingLevel;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::Score;
use crate::resources::prelude::*;
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct LightCavalry;

impl LightCavalry {
  pub const ID: UnitId = UnitId::LightCavalry;
  pub const KIND: UnitKind = UnitKind::Cavalry;

  pub const SCORE: Score = Score::new(4);

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(Power::new(130))
    .infantry_defense(Power::new(30))
    .cavalry_defense(Power::new(40))
    .ranged_defense(Power::new(30))
    .ranged_debuff(RangedDebuff::MIN)
    .speed(Speed::new(10.0))
    .haul(Haul::new(80))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(5),
    cost: Cost::new(2375),
    wood_ratio: ResourceRatio::new(0.3),
    stone_ratio: ResourceRatio::new(0.2),
    iron_ratio: ResourceRatio::new(0.5),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(2),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .stable(BuildingLevel::new(1))
      .build();
}

check_total_resource_ratio!(
  LightCavalry::CHUNK.wood_ratio,
  LightCavalry::CHUNK.stone_ratio,
  LightCavalry::CHUNK.iron_ratio
);
