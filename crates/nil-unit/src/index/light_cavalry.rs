use super::prelude::*;

#[derive(Unit)]
pub struct LightCavalry {
  amount: u32,
}

impl LightCavalry {
  pub const ID: UnitId = UnitId::new(5);
  pub const KIND: UnitKind = UnitKind::Cavalry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(130),
    general_defense: Power::new(30),
    cavalry_defense: Power::new(40),
    speed: Speed::new(10.0),
    haul: Haul::new(80),
  };
}
