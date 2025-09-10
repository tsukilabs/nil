// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::infrastructure::building::wall::WallStats;
use crate::infrastructure::prelude::BuildingLevel;
use crate::military::army::ArmyPersonnel;
use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::UnitKind;
use bon::Builder;

#[derive(Builder)]
pub struct Battle<'a> {
  #[builder(default)]
  attacker: &'a [Squad],
  #[builder(default)]
  defender: &'a [Squad],
  wall: Option<&'a WallStats>,
}

impl Battle<'_> {
  pub fn offensive_power(&self) -> OffensivePower {
    OffensivePower::new(self.attacker)
  }

  pub fn defensive_power(&self) -> DefensivePower {
    DefensivePower::new(self.defender, &self.offensive_power(), self.wall)
  }

  pub fn battle_result(&self) -> BattleResult {
    BattleResult::new(self.attacker, self.defender, self.wall)
  }
}

#[derive(Clone, Debug)]
enum BattleWinner {
  Attacker,
  Defender,
}

#[derive(Clone, Debug)]
pub struct OffensivePower {
  total: f64,
  infantry_ratio: f64,
  cavalry_ratio: f64,
  ranged_ratio: f64,
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
    }
  }
}

#[expect(dead_code)]
#[derive(Clone, Debug)]
pub struct DefensivePower {
  total: f64,
  infantry_ratio: f64,
  cavalry_ratio: f64,
  ranged_ratio: f64,
  ranged_is_debuffed: bool,
}

impl DefensivePower {
  pub fn new(
    squads: &[Squad],
    offensive_power: &OffensivePower,
    defending_wall: Option<&WallStats>,
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
    let mut ranged_is_debuffed = false;

    if f64::from(units_by_kind.ranged) / f64::from(units_by_kind.units_amount) > 0.5 {
      total -= sum_ranged_debuff(squads);
      ranged_is_debuffed = true;
    }

    if let Some(wall_power) = defending_wall {
      total = add_wall_power(wall_power, total);
    }

    DefensivePower {
      total,
      infantry_ratio: infantry / total,
      cavalry_ratio: cavalry / total,
      ranged_ratio: ranged / total,
      ranged_is_debuffed,
    }
  }
}

#[expect(dead_code)]
#[derive(Clone, Debug)]
pub struct BattleResult {
  attacker_personnel: ArmyPersonnel,
  attacker_surviving_personnel: ArmyPersonnel,
  defender_personnel: ArmyPersonnel,
  defender_surviving_personnel: ArmyPersonnel,
  wall_level: BuildingLevel,
}

impl BattleResult {
  fn new(defending_squads: &[Squad], attacking_squads: &[Squad], wall: Option<&WallStats>) -> Self {
    let attacker_power = OffensivePower::new(attacking_squads);
    let defender_power = DefensivePower::new(defending_squads, &attacker_power, wall);

    let winner = determine_winner(&attacker_power, &defender_power);

    let attacker_personnel: ArmyPersonnel = attacking_squads.iter().cloned().collect();
    let defender_personnel: ArmyPersonnel = defending_squads.iter().cloned().collect();

    let mut attacker_surviving_personnel = ArmyPersonnel::default();
    let mut defender_surviving_personnel = ArmyPersonnel::default();

    let losses_ratio = match winner {
      BattleWinner::Attacker => (defender_power.total / attacker_power.total).powf(1.5),
      BattleWinner::Defender => (attacker_power.total / defender_power.total).powf(1.5),
    };

    let mut squad_survivors: f64;

    match winner {
      BattleWinner::Attacker => {
        for squad in attacking_squads {
          let squad_size = f64::from(squad.size());
          squad_survivors = squad_size - (squad_size * losses_ratio);
          attacker_surviving_personnel += Squad::new(squad.id(), SquadSize::from(squad_survivors));
        }
      }
      BattleWinner::Defender => {
        for squad in defending_squads {
          let squad_size = f64::from(squad.size());
          squad_survivors = squad_size - (squad_size * losses_ratio);
          defender_surviving_personnel += Squad::new(squad.id(), SquadSize::from(squad_survivors));
        }
      }
    }

    let wall_level = wall
      .map(|stats| stats.level)
      .unwrap_or_default();

    BattleResult {
      attacker_personnel,
      attacker_surviving_personnel,
      defender_personnel,
      defender_surviving_personnel,
      wall_level,
    }
  }
}

fn determine_winner(attacker: &OffensivePower, defender: &DefensivePower) -> BattleWinner {
  let attacker_power = attacker.total;
  let defender_power = defender.total;
  if attacker_power > defender_power {
    BattleWinner::Attacker
  } else {
    BattleWinner::Defender
  }
}

#[derive(Copy, Clone, Debug)]
#[expect(dead_code)]
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
