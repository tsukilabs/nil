// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Haul, Power, RangedDebuff, Speed, UnitId, UnitKind, UnitStats};
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct Swordsman;

impl Swordsman {
  pub const ID: UnitId = UnitId::Swordsman;
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(25),
    infantry_defense: Power::new(50),
    cavalry_defense: Power::new(15),
    ranged_defense: Power::new(40),
    ranged_debuff: RangedDebuff::new(0.0),
    speed: Speed::new(22.0),
    haul: Haul::new(15),
  };
}
