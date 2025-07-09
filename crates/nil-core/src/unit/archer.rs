// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Haul, Power, RangedDebuff, Speed, UnitId, UnitKind, UnitStats};
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct Archer;

impl Archer {
  pub const ID: UnitId = UnitId::Archer;
  pub const KIND: UnitKind = UnitKind::Ranged;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(30),
    infantry_defense: Power::new(50),
    cavalry_defense: Power::new(40),
    ranged_defense: Power::new(5),
    ranged_debuff: RangedDebuff::new(5.0),
    speed: Speed::new(18.0),
    haul: Haul::new(10),
  };
}
