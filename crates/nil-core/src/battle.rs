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

  pub fn defensive_power(&self) -> DefensivePower {
    DefensivePower::new(&self.defender, self.offensive_power())
  }

  pub fn winner_losses(&self) -> WinnerLosses {
    WinnerLosses::new(self.offensive_power(), self.defensive_power())
  }
}

enum BattleWinner {
  Attacker,
  Defender,
}

#[derive(Copy, Clone, Debug)]
pub struct OffensivePower {
  pub total: f64,
  pub infantry_ratio: f64,
  pub cavalry_ratio: f64,
  pub ranged_ratio: f64,
  pub  units_by_kind: UnitsByKind,
}

impl OffensivePower {
  pub fn new(units: &[UnitBox]) -> Self {
    let units_by_kind = units_by_kind(units);
    let mut infantry = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    for unit in units {
      match unit.kind() {
        UnitKind::Infantry => {
          infantry += *unit.squad_attack();
        }
        UnitKind::Cavalry => {
          cavalry += *unit.squad_attack();
        }
        UnitKind::Ranged => {
          ranged += *unit.squad_attack();
        }
      }
    }

    let total = infantry + cavalry + ranged;

    OffensivePower {
      total,
      infantry_ratio: infantry / total,
      cavalry_ratio: cavalry / total,
      ranged_ratio: ranged / total,
      units_by_kind,
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct DefensivePower {
  pub total: f64,
  pub infantry_ratio: f64,
  pub cavalry_ratio: f64,
  pub ranged_ratio: f64,
  pub units_by_kind: UnitsByKind,
}

impl DefensivePower {
  pub fn new(units: &[UnitBox], offensive_power: OffensivePower) -> Self {
    let units_by_kind = units_by_kind(units);
    let mut infantry = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    for unit in units {
      infantry += unit.squad_defense().infantry;
      cavalry += unit.squad_defense().cavalry;
      ranged += unit.squad_defense().ranged;
    }

    infantry *= offensive_power.infantry_ratio;
    cavalry *= offensive_power.cavalry_ratio;
    ranged *= offensive_power.ranged_ratio;

    let total = infantry + cavalry + ranged;

    DefensivePower {
      total,
      infantry_ratio: infantry / total,
      cavalry_ratio: cavalry / total,
      ranged_ratio: ranged / total,
      units_by_kind,
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct WinnerLosses {
  pub total_loss: f64,
  pub infantry: f64,
  pub cavalry_losses: f64,
  pub ranged_losses: f64,
}

impl WinnerLosses {
  pub fn new(offensive_power: OffensivePower, defesive_power: DefensivePower) -> Self {
    let winner = determine_winner(offensive_power, defesive_power);
    let winner_power = match winner {
      BattleWinner::Attacker => offensive_power.total,
      BattleWinner::Defender => defesive_power.total,
    };
    let loser_power = match winner {
      BattleWinner::Attacker => defesive_power.total,
      BattleWinner::Defender => offensive_power.total,
    };

    let winner_units = match winner {
      BattleWinner::Attacker => offensive_power.units_by_kind.units_amount,
      BattleWinner::Defender => defesive_power.units_by_kind.units_amount,
    };

    let winner_infantry = match winner {
      BattleWinner::Attacker => offensive_power.units_by_kind.infantry,
      BattleWinner::Defender => defesive_power.units_by_kind.infantry,
    };

    let winner_cavalry = match winner {
      BattleWinner::Attacker => offensive_power.units_by_kind.cavalry,
      BattleWinner::Defender => defesive_power.units_by_kind.cavalry,
    };
      

    let winner_ranged = match winner {
      BattleWinner::Attacker => offensive_power.units_by_kind.ranged,
      BattleWinner::Defender => defesive_power.units_by_kind.ranged,
    };

    let losses_percent = 100.0 * (loser_power / winner_power).powf(1.5);

    WinnerLosses {
      total_loss: (f64::from(winner_units) / 100.0) * losses_percent,
      infantry: (f64::from(winner_infantry) / 100.0) * losses_percent,
      cavalry_losses: (f64::from(winner_cavalry) / 100.0) * losses_percent,
      ranged_losses: (f64::from(winner_ranged) / 100.0) * losses_percent,
    }
  }
}

fn determine_winner(attacker: OffensivePower, defender: DefensivePower) -> BattleWinner {
  let attacker_power = attacker.total;
  let defender_power = defender.total;
  if attacker_power > defender_power {
    BattleWinner::Attacker
  } else {
    BattleWinner::Defender
  }
}

#[derive(Copy, Clone, Debug)]
pub struct UnitsByKind {
  infantry: u32,
  cavalry: u32,
  ranged: u32,
  units_amount: u32,
}

  fn units_by_kind(units: &[UnitBox]) -> UnitsByKind{
    let mut infantry = 0;
    let mut cavalry = 0;
    let mut ranged = 0;
    let mut units_amount = 0;
  
    for unit in units {
      match unit.kind() {
        UnitKind::Infantry => {
          infantry += unit.amount();
        }
        UnitKind::Cavalry => {
          cavalry += unit.amount();
        }
        UnitKind::Ranged => {
          ranged += unit.amount();
        }
      }
      units_amount += unit.amount();
    }
  
    UnitsByKind {
      infantry,
      cavalry,
      ranged,
      units_amount,
    }
  }
