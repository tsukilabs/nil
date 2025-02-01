use crate::unit::{UnitBox, UnitKind};
use bon::Builder;

#[derive(Builder)]
pub struct Battle {
  #[builder(default, into)]
  attacker: Vec<UnitBox>,
  #[builder(default, into)]
  defender: Vec<UnitBox>,
}

impl Battle {
  pub fn offensive_power(&self) -> OffensivePower {
    OffensivePower::new(&self.attacker)
  }
}

#[derive(Copy, Clone, Debug)]
pub struct OffensivePower {
  pub total: f64,
  pub infantry_ratio: f64,
  pub cavalry_ratio: f64,
  pub ranged_ratio: f64,
}

impl OffensivePower {
  pub fn new(units: &[UnitBox]) -> Self {
    let mut infantry = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    for unit in units {
      match unit.kind() {
        UnitKind::Infantry => {
          infantry += unit.sum_attack();
        }
        UnitKind::Cavalry => {
          cavalry += unit.sum_attack();
        }
        UnitKind::Ranged => {
          ranged += unit.sum_attack();
        }
      }
    }

    let total = infantry + cavalry + ranged;

    OffensivePower {
      total,
      infantry_ratio: infantry / total,
      cavalry_ratio: cavalry / total,
      ranged_ratio: ranged / total,
    }
  }
}
