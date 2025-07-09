// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Haul, Power, RangedDebuff, Speed, UnitId, UnitKind, UnitStats};
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct HeavyCavalry;

impl HeavyCavalry {
  pub const ID: UnitId = UnitId::HeavyCavalry;
  pub const KIND: UnitKind = UnitKind::Cavalry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(150),
    infantry_defense: Power::new(200),
    cavalry_defense: Power::new(80),
    ranged_defense: Power::new(180),
    ranged_debuff: RangedDebuff::new(0.0),
    speed: Speed::new(11.0),
    haul: Haul::new(50),
  };
}
