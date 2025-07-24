// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::infrastructure::building::wall::WallStats;
use crate::military::squad::Squad;
use crate::military::unit::UnitKind;
use bon::Builder;

#[derive(Builder)]
pub struct Battle {
  #[builder(default, into)]
  attacker: Vec<Squad>,
  #[builder(default, into)]
  defender: Vec<Squad>,
  wall: Option<WallStats>,
}

impl Battle {
  pub fn offensive_power(&self) -> OffensivePower {
    OffensivePower::new(&self.attacker)
  }

  pub fn defensive_power(&self) -> DefensivePower {
    DefensivePower::new(&self.defender, self.offensive_power(), &self.wall)
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
  total: f64,
  infantry_ratio: f64,
  cavalry_ratio: f64,
  ranged_ratio: f64,
  units_by_kind: UnitsByKind,
}

impl OffensivePower {
  fn new(squads: &[Squad]) -> Self {
    let units_by_kind = UnitsByKind::new(squads);
    let mut infantry = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    for squad in squads {
      match squad.kind() {
        UnitKind::Infantry => {
          infantry += *squad.attack();
        }
        UnitKind::Cavalry => {
          cavalry += *squad.attack();
        }
        UnitKind::Ranged => {
          ranged += *squad.attack();
        }
      }
    }

    if f64::from(units_by_kind.ranged) / f64::from(units_by_kind.units_amount) > 0.3 {
      ranged -= sum_ranged_debuff(squads);
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

#[expect(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct DefensivePower {
  total: f64,
  infantry_ratio: f64,
  cavalry_ratio: f64,
  ranged_ratio: f64,
  units_by_kind: UnitsByKind,
}

impl DefensivePower {
  pub fn new(
    squads: &[Squad],
    offensive_power: OffensivePower,
    defending_wall: &Option<WallStats>,
  ) -> Self {
    let units_by_kind = UnitsByKind::new(squads);
    let mut infantry = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    for squad in squads {
      infantry += squad.defense().infantry;
      cavalry += squad.defense().cavalry;
      ranged += squad.defense().ranged;
    }

    infantry *= offensive_power.infantry_ratio;
    cavalry *= offensive_power.cavalry_ratio;
    ranged *= offensive_power.ranged_ratio;

    let mut total = infantry + cavalry + ranged;

    if f64::from(units_by_kind.ranged) / f64::from(units_by_kind.units_amount) > 0.5 {
      total -= sum_ranged_debuff(squads);
    }

    if let Some(wall_power) = defending_wall {
      total = add_wall_power(wall_power, total);
    }

    DefensivePower {
      total,
      infantry_ratio: infantry / total,
      cavalry_ratio: cavalry / total,
      ranged_ratio: ranged / total,
      units_by_kind,
    }
  }
}

#[expect(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct WinnerLosses {
  total_loss: f64,
  infantry: f64,
  cavalry_losses: f64,
  ranged_losses: f64,
}

impl WinnerLosses {
  fn new(offensive_power: OffensivePower, defesive_power: DefensivePower) -> Self {
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

impl UnitsByKind {
  fn new(squads: &[Squad]) -> Self {
    let mut infantry = 0;
    let mut cavalry = 0;
    let mut ranged = 0;
    let mut units_amount = 0;

    for squad in squads {
      let amount = *squad.size();
      match squad.kind() {
        UnitKind::Infantry => infantry += amount,
        UnitKind::Cavalry => cavalry += amount,
        UnitKind::Ranged => ranged += amount,
      }

      units_amount += amount;
    }

    Self {
      infantry,
      cavalry,
      ranged,
      units_amount,
    }
  }
}

fn sum_ranged_debuff(squads: &[Squad]) -> f64 {
  let mut ranged_debuff = 0.0;
  for squad in squads {
    if squad.kind() == UnitKind::Ranged {
      let size = *squad.size();
      let stats = squad.unit().stats();
      ranged_debuff += stats.ranged_debuff() * size;
    }
  }
  ranged_debuff
}

fn add_wall_power(wall: &WallStats, current_power: f64) -> f64 {
  current_power
    + f64::from(wall.defense)
    + ((f64::from(wall.defense_percent) / 100.0) * current_power)
}
