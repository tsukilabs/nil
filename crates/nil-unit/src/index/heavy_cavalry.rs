use super::prelude::*;

#[derive(Unit)]
pub struct HeavyCavalry {
  amount: u32,
}

impl HeavyCavalry {
  pub const ID: UnitId = UnitId::new(6);
  pub const KIND: UnitKind = UnitKind::Cavalry;
  pub const STATS: UnitStats = UnitStats {
    attack: Power::new(150),
    infantry_defense: Power::new(200),
    cavalry_defense: Power::new(80),
    ranged_defense: Power::new(180),
    speed: Speed::new(11.0),
    haul: Haul::new(50),
  };
}
