use super::{Haul, Power, Speed, UnitId, UnitKind, UnitStats};
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct Archer;

impl Archer {
  pub const ID: UnitId = UnitId::new(4);
  pub const KIND: UnitKind = UnitKind::Ranged;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(30),
    infantry_defense: Power::new(50),
    cavalry_defense: Power::new(40),
    ranged_defense: Power::new(5),
    ranged_debuff: 5.0,
    speed: Speed::new(18.0),
    haul: Haul::new(10),
  };
}
