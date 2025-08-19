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
pub struct Archer;

impl Archer {
  pub const ID: UnitId = UnitId::Archer;
  pub const KIND: UnitKind = UnitKind::Ranged;

  pub const SCORE: Score = Score::new(1);

  pub const STATS: UnitStats = UnitStats::builder()
    .attack(Power::new(30))
    .infantry_defense(Power::new(50))
    .cavalry_defense(Power::new(40))
    .ranged_defense(Power::new(5))
    .ranged_debuff(RangedDebuff::new(5.0))
    .speed(Speed::new(18.0))
    .haul(Haul::new(10))
    .build();

  pub const CHUNK: UnitChunk = UnitChunk {
    size: UnitChunkSize::new(10),
    cost: Cost::new(1900),
    wood_ratio: ResourceRatio::new(0.5),
    stone_ratio: ResourceRatio::new(0.15),
    iron_ratio: ResourceRatio::new(0.35),
    maintenance_ratio: MaintenanceRatio::new(0.005),
    workforce: Workforce::new(1),
  };

  pub const INFRASTRUCTURE_REQUIREMENTS: InfrastructureRequirements =
    InfrastructureRequirements::builder()
      .academy(BuildingLevel::new(10))
      .build();
}

check_total_resource_ratio!(
  Archer::CHUNK.wood_ratio,
  Archer::CHUNK.stone_ratio,
  Archer::CHUNK.iron_ratio
);
