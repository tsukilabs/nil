use super::{Haul, Power, Speed, UnitId, UnitKind, UnitStats};
use nil_core_macros::Unit;

#[derive(Unit)]
pub struct Pikeman;

impl Pikeman {
  pub const ID: UnitId = UnitId::new(1);
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(10),
    infantry_defense: Power::new(15),
    cavalry_defense: Power::new(45),
    ranged_defense: Power::new(20),
    ranged_debuff: 0.0,
    speed: Speed::new(18.0),
    haul: Haul::new(25),
  };
}
