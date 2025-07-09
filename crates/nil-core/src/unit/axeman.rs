// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Haul, Power, RangedDebuff, Speed, UnitId, UnitKind, UnitStats};
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct Axeman;

impl Axeman {
  pub const ID: UnitId = UnitId::Axeman;
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(40),
    infantry_defense: Power::new(10),
    cavalry_defense: Power::new(5),
    ranged_defense: Power::new(10),
    ranged_debuff: RangedDebuff::new(0.0),
    speed: Speed::new(18.0),
    haul: Haul::new(10),
  };
}
