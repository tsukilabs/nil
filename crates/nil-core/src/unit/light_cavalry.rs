use super::{Haul, Power, Speed, UnitId, UnitKind, UnitStats};
use nil_macros::Unit;

#[derive(Unit)]
pub struct LightCavalry;

impl LightCavalry {
  pub const ID: UnitId = UnitId::new(5);
  pub const KIND: UnitKind = UnitKind::Cavalry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(130),
    infantry_defense: Power::new(30),
    cavalry_defense: Power::new(40),
    ranged_defense: Power::new(30),
    ranged_debuff: 0.0,
    speed: Speed::new(10.0),
    haul: Haul::new(80),
  };
}
