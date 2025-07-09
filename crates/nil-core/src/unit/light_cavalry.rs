// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Haul, Power, RangedDebuff, Speed, UnitId, UnitKind, UnitStats};
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct LightCavalry;

impl LightCavalry {
  pub const ID: UnitId = UnitId::LightCavalry;
  pub const KIND: UnitKind = UnitKind::Cavalry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(130),
    infantry_defense: Power::new(30),
    cavalry_defense: Power::new(40),
    ranged_defense: Power::new(30),
    ranged_debuff: RangedDebuff::new(0.0),
    speed: Speed::new(10.0),
    haul: Haul::new(80),
  };
}
