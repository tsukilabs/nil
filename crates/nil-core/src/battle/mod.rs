// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod luck;

#[cfg(test)]
mod tests;

use crate::infrastructure::building::wall::WallStats;
use crate::infrastructure::prelude::BuildingLevel;
use crate::military::army::ArmyPersonnel;
use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::UnitKind;
use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder)]
pub struct Battle<'a> {
  #[builder(default)]
  attacker: &'a [Squad],

  #[builder(default)]
  defender: &'a [Squad],

  wall: Option<&'a WallStats>,
}

impl Battle<'_> {
  #[inline]
  pub fn result(self) -> BattleResult {
    BattleResult::new(self.attacker, self.defender, self.wall)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleResult {
  attacker_personnel: ArmyPersonnel,
  attacker_surviving_personnel: ArmyPersonnel,
  defender_personnel: ArmyPersonnel,
  defender_surviving_personnel: ArmyPersonnel,
  wall_level: BuildingLevel,
  winner: BattleWinner,
}

impl BattleResult {
  #[rustfmt::skip]
  fn new(attacking_squads: &[Squad], defending_squads: &[Squad], wall: Option<&WallStats>) -> Self {
    let attacker_power = OffensivePower::new(attacking_squads);
    let defender_power = DefensivePower::new(defending_squads, &attacker_power, wall);

    let winner = BattleWinner::determine(&attacker_power, &defender_power);

    let attacker_personnel: ArmyPersonnel = attacking_squads.iter().cloned().collect();
    let defender_personnel: ArmyPersonnel = defending_squads.iter().cloned().collect();

    let mut attacker_surviving_personnel = ArmyPersonnel::default();
    let mut defender_surviving_personnel = ArmyPersonnel::default();

    let losses_ratio = match winner {
      BattleWinner::Attacker => defender_power.total / attacker_power.total,
      BattleWinner::Defender => attacker_power.total / defender_power.total,
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
      winner,
    }
  }

  #[inline]
  pub fn attacker_personnel(&self) -> &ArmyPersonnel {
    &self.attacker_personnel
  }

  #[inline]
  pub fn attacker_surviving_personnel(&self) -> &ArmyPersonnel {
    &self.attacker_surviving_personnel
  }

  #[inline]
  pub fn defender_personnel(&self) -> &ArmyPersonnel {
    &self.defender_personnel
  }

  #[inline]
  pub fn defender_surviving_personnel(&self) -> &ArmyPersonnel {
    &self.defender_surviving_personnel
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BattleWinner {
  Attacker,
  Defender,
}

impl BattleWinner {
  fn determine(attacker: &OffensivePower, defender: &DefensivePower) -> Self {
    if attacker.total > defender.total { Self::Attacker } else { Self::Defender }
  }
}

struct OffensivePower {
  total: f64,
  infantry: f64,
  cavalry: f64,
  ranged: f64,
}

impl OffensivePower {
  fn new(squads: &[Squad]) -> Self {
    let mut infantry = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    let mut army_size = 0.0;
    let mut ranged_size = 0.0;

    for squad in squads {
      army_size += f64::from(squad.size());
      match squad.kind() {
        UnitKind::Infantry => {
          infantry += *squad.attack();
        }
        UnitKind::Cavalry => {
          cavalry += *squad.attack();
        }
        UnitKind::Ranged => {
          ranged += *squad.attack();
          ranged_size += f64::from(squad.size());
        }
      }
    }

    if ranged_size / army_size > 0.3 {
      ranged *= sum_ranged_debuff(squads);
    }

    let total = infantry + cavalry + ranged;

    OffensivePower { total, infantry, cavalry, ranged }
  }
}

struct DefensivePower {
  total: f64,
}

impl DefensivePower {
  fn new(
    squads: &[Squad],
    offensive_power: &OffensivePower,
    defending_wall: Option<&WallStats>,
  ) -> Self {
    let mut infantry_power = 0.0;
    let mut cavalry_power = 0.0;
    let mut ranged_power = 0.0;

    let mut army_size = 0.0;
    let mut ranged_squad_size = 0.0;

    for squad in squads {
      if squad.kind() == UnitKind::Ranged {
        ranged_squad_size += f64::from(squad.size());
      }
      army_size += f64::from(squad.size());
    }

    for squad in squads {
      if squad.kind() == UnitKind::Ranged && ranged_squad_size / army_size > 0.5 {
        infantry_power += squad.defense().infantry * sum_ranged_debuff(squads);
        cavalry_power += squad.defense().cavalry * sum_ranged_debuff(squads);
        ranged_power += squad.defense().ranged * sum_ranged_debuff(squads);
      } else {
        infantry_power += squad.defense().infantry;
        cavalry_power += squad.defense().cavalry;
        ranged_power += squad.defense().ranged;
      }
    }

    let mut total = 0.0;

    if army_size > 0.0 {
      let infantry_power_per_unit = infantry_power / army_size;
      let cavalry_power_per_unit = cavalry_power / army_size;
      let ranged_power_per_unit = ranged_power / army_size;

      let mut infantry_necessary_units = offensive_power.infantry / infantry_power_per_unit;
      let mut cavalry_necessary_units = offensive_power.cavalry / cavalry_power_per_unit;
      let mut ranged_necessary_units = offensive_power.ranged / ranged_power_per_unit;

      let necessary_units =
        infantry_necessary_units + cavalry_necessary_units + ranged_necessary_units;

      infantry_necessary_units /= necessary_units;
      cavalry_necessary_units /= necessary_units;
      ranged_necessary_units /= necessary_units;

      infantry_power = infantry_necessary_units * army_size * infantry_power_per_unit;
      cavalry_power = cavalry_necessary_units * army_size * cavalry_power_per_unit;
      ranged_power = ranged_necessary_units * army_size * ranged_power_per_unit;

      total = infantry_power + cavalry_power + ranged_power;
    }

    if let Some(wall_power) = defending_wall {
      total = add_wall_power(wall_power, total);
    }

    DefensivePower { total }
  }
}

fn sum_ranged_debuff(squads: &[Squad]) -> f64 {
  let mut ranged_debuff = 0.0;
  let mut ranged_amount = 0;
  for squad in squads {
    if squad.kind() == UnitKind::Ranged {
      let size = *squad.size();
      let stats = squad.unit().stats();
      ranged_debuff += stats.ranged_debuff() * size;
      ranged_amount += size;
    }
  }
  ranged_debuff / f64::from(ranged_amount)
}

fn add_wall_power(wall: &WallStats, current_power: f64) -> f64 {
  current_power
    + f64::from(wall.defense)
    + ((f64::from(wall.defense_percent) / 100.0) * current_power)
}
