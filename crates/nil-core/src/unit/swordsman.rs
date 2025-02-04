use super::{Haul, Power, Speed, UnitId, UnitKind, UnitStats};
use nil_macros::Unit;

#[derive(Unit)]
pub struct Swordsman;

impl Swordsman {
  pub const ID: UnitId = UnitId::new(2);
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(25),
    infantry_defense: Power::new(50),
    cavalry_defense: Power::new(15),
    ranged_defense: Power::new(40),
    ranged_debuff: 0.0,
    speed: Speed::new(22.0),
    haul: Haul::new(15),
  };
}
