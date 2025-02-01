use super::prelude::*;

#[derive(Unit)]
pub struct Pikeman {
  amount: u32,
}

impl Pikeman {
  pub const ID: UnitId = UnitId::new(1);
  pub const KIND: UnitKind = UnitKind::Infantry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(10),
    general_defense: Power::new(15),
    cavalry_defense: Power::new(45),
    speed: Speed::new(18.0),
    haul: Haul::new(25),
  };
}
