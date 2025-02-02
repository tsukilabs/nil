use super::prelude::*;

#[derive(Unit)]
pub struct Archer {
  amount: u32,
}

impl Archer {
  pub const ID: UnitId = UnitId::new(4);
  pub const KIND: UnitKind = UnitKind::Ranged;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(15),
    infantry_defense: Power::new(50),
    cavalry_defense: Power::new(40),
    ranged_defense: Power::new(5),
    speed: Speed::new(18.0),
    haul: Haul::new(10),
  };
}
