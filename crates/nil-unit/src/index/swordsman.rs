use super::prelude::*;

#[derive(Unit)]
pub struct Swordsman {
  amount: u32,
}

impl Swordsman {
  pub const ID: UnitId = UnitId::new(2);
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(25),
    general_defense: Power::new(50),
    cavalry_defense: Power::new(15),
    speed: Speed::new(22.0),
    haul: Haul::new(15),
  };
}
