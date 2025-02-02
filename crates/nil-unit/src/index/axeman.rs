use super::prelude::*;

#[derive(Unit)]
pub struct Axeman {
  amount: u32,
}

impl Axeman {
  pub const ID: UnitId = UnitId::new(3);
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(40),
    infantry_defense: Power::new(10),
    cavalry_defense: Power::new(5),
    ranged_defense: Power::new(10),
    speed: Speed::new(18.0),
    haul: Haul::new(10),
  };
}
