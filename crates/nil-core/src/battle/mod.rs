// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod luck;

#[cfg(test)]
mod tests;

use crate::error::Result;
use crate::infrastructure::building::wall::{Wall, WallStats};
use crate::infrastructure::building::{BuildingLevel, BuildingLevelDiff};
use crate::infrastructure::stats::InfrastructureStats;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::squad::Squad;
use crate::military::unit::{UnitId, UnitKind};
use bon::Builder;
use luck::Luck;
use nil_num::growth::growth;
use serde::{Deserialize, Serialize};

#[derive(Builder)]
pub struct Battle<'a> {
  #[builder(default)]
  attacker: &'a [Squad],

  #[builder(default)]
  defender: &'a [Squad],

  #[builder(default)]
  luck: Luck,

  wall: Option<&'a WallStats>,

  infrastructure_stats: &'a InfrastructureStats,
}

impl Battle<'_> {
  #[inline]
  pub fn result(self) -> Result<BattleResult> {
    BattleResult::new(
      self.attacker,
      self.defender,
      self.luck,
      self.wall,
      self.infrastructure_stats,
    )
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
  downgraded_wall_level: BuildingLevelDiff,
  winner: BattleWinner,
  luck: Luck,
}

impl BattleResult {
  fn new(
    attacking_squads: &[Squad],
    defending_squads: &[Squad],
    luck: Luck,
    wall: Option<&WallStats>,
    infrastructure_stats: &InfrastructureStats,
  ) -> Result<Self> {
    let attacker_power = OffensivePower::new(attacking_squads, luck);
    let defender_power = DefensivePower::new(
      defending_squads,
      &attacker_power,
      wall,
      infrastructure_stats,
    )?;

    let winner = BattleWinner::determine(&attacker_power, &defender_power);

    let attacker_personnel: ArmyPersonnel = attacking_squads.iter().cloned().collect();
    let defender_personnel: ArmyPersonnel = defending_squads.iter().cloned().collect();

    let mut attacker_surviving_personnel = ArmyPersonnel::default();
    let mut defender_surviving_personnel = ArmyPersonnel::default();

    let wall_level = wall
      .map(|stats| stats.level)
      .unwrap_or_default();

    let mut downgraded_wall_level = BuildingLevelDiff::new(0);

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
          attacker_surviving_personnel += Squad::new(squad.id(), squad_survivors);
        }

        if wall_level > 0 && attacker_power.rams_amount > 0.0 {
          let remaining_rams =
            attacker_power.rams_amount - (attacker_power.rams_amount * losses_ratio);
          let wall_levels_to_decrease = wall_level * (remaining_rams / 200.0 - losses_ratio) * 0.5;

          if wall_levels_to_decrease > Wall::MAX_LEVEL {
            downgraded_wall_level = -Wall::MAX_LEVEL;
          } else if wall_levels_to_decrease > 0.0 {
            downgraded_wall_level = BuildingLevelDiff::from(-wall_levels_to_decrease);
          }
        }
      }
      BattleWinner::Defender => {
        for squad in defending_squads {
          let squad_size = f64::from(squad.size());
          squad_survivors = squad_size - (squad_size * losses_ratio);
          defender_surviving_personnel += Squad::new(squad.id(), squad_survivors);
        }

        if wall_level > 0 && attacker_power.rams_amount > 0.0 {
          let diff = -(wall_level * losses_ratio * 0.3);
          downgraded_wall_level = BuildingLevelDiff::from(diff);
        }
      }
    }

    Ok(BattleResult {
      attacker_personnel,
      attacker_surviving_personnel,
      defender_personnel,
      defender_surviving_personnel,
      wall_level,
      downgraded_wall_level,
      winner,
      luck,
    })
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

  pub fn defender_surviving_personnel_ratio(&self) -> f64 {
    let total = self
      .defender_personnel
      .iter()
      .map(|squad| f64::from(squad.size()))
      .sum::<f64>();

    let surviving = self
      .defender_surviving_personnel
      .iter()
      .map(|squad| f64::from(squad.size()))
      .sum::<f64>();

    if total > 0.0 { surviving / total } else { 0.0 }
  }

  #[inline]
  pub fn wall_level(&self) -> BuildingLevel {
    self.wall_level
  }

  #[inline]
  pub fn downgraded_wall_level(&self) -> BuildingLevelDiff {
    self.downgraded_wall_level
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
  rams_amount: f64,
}

impl OffensivePower {
  fn new(squads: &[Squad], luck: Luck) -> Self {
    let mut infantry = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;
    let mut rams_amount = 0.0;
    let mut ranged_with_debuff = 0.0;

    let mut army_size = 0.0;
    let mut ranged_amount = 0.0;

    for squad in squads {
      army_size += f64::from(squad.size());
      match squad.kind() {
        UnitKind::Infantry => {
          infantry += *squad.attack();
          if squad.id() == UnitId::Ram {
            rams_amount = f64::from(squad.size());
          }
        }
        UnitKind::Cavalry => {
          cavalry += *squad.attack();
        }
        UnitKind::Ranged => {
          ranged += *squad.attack();
          ranged_with_debuff += *squad.attack() * f64::from(squad.unit().stats().ranged_debuff());
          ranged_amount += f64::from(squad.size());
        }
      }
    }

    if ranged_amount / army_size > 0.3 {
      ranged = ranged_with_debuff;
    }
    infantry += infantry * luck;
    cavalry += cavalry * luck;
    ranged += ranged * luck;

    let total = infantry + cavalry + ranged;

    OffensivePower {
      total,
      infantry,
      cavalry,
      ranged,
      rams_amount,
    }
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
    infrastructure_stats: &InfrastructureStats,
  ) -> Result<Self> {
    let mut infantry = 0.0;
    let mut cavalry = 0.0;
    let mut ranged = 0.0;

    let mut army_size = 0.0;

    for squad in squads {
      infantry += squad.defense().infantry;
      cavalry += squad.defense().cavalry;
      ranged += squad.defense().ranged;

      army_size += f64::from(squad.size());
    }

    let mut total = 0.0;

    if army_size > 0.0 {
      let infantry_power_per_unit = infantry / army_size;
      let cavalry_power_per_unit = cavalry / army_size;
      let ranged_power_per_unit = ranged / army_size;

      let infantry_necessary_units = offensive_power.infantry / infantry_power_per_unit;
      let cavalry_necessary_units = offensive_power.cavalry / cavalry_power_per_unit;
      let ranged_necessary_units = offensive_power.ranged / ranged_power_per_unit;

      let necessary_units =
        infantry_necessary_units + cavalry_necessary_units + ranged_necessary_units;

      let infantry_proportion = infantry_necessary_units / necessary_units;
      let cavalry_proportion = cavalry_necessary_units / necessary_units;
      let ranged_proportion = ranged_necessary_units / necessary_units;

      infantry = infantry_proportion * army_size * infantry_power_per_unit;
      cavalry = cavalry_proportion * army_size * cavalry_power_per_unit;
      ranged = ranged_proportion * army_size * ranged_power_per_unit;

      total = infantry + cavalry + ranged;
    }

    if let Some(wall) = defending_wall {
      let mut attacking_rams = offensive_power.rams_amount;

      let surviving_rams_no_wall = offensive_power.rams_amount
        - (offensive_power.rams_amount * (total / offensive_power.total));
      attacking_rams = attacking_rams + (surviving_rams_no_wall * 2.0);

      if attacking_rams > 0.0 {
        let rams_growth_per_wall_level: f64 = growth()
          .floor(wall.level)
          .ceil(650)
          .max_level(Wall::MAX_LEVEL)
          .call();

        let mut rams_vec = Vec::new();
        let mut rams_per_wall_level = f64::from(wall.level);
        let mut total_of_rams = 0.0;

        for _ in 1..=usize::from(wall.level) {
          rams_vec.push(rams_per_wall_level - total_of_rams);
          total_of_rams = rams_per_wall_level;
          rams_per_wall_level += rams_per_wall_level * rams_growth_per_wall_level;
        }

        let mut wall_levels_to_decrease: u8 = 0;
        for value in rams_vec.iter().rev() {
          if attacking_rams >= *value && wall_levels_to_decrease < wall.level {
            attacking_rams -= value;
            wall_levels_to_decrease += 1;
          } else {
            break;
          }
        }

        if wall.level - wall_levels_to_decrease > 0 {
          let new_wall = infrastructure_stats
            .wall()
            .get(wall.level - wall_levels_to_decrease)?;

          total += new_wall.defense + ((new_wall.defense_percent / 100.0) * total);
        }
      } else if wall.level > 0 {
        total += wall.defense + ((wall.defense_percent / 100.0) * total);
      }
    }

    Ok(DefensivePower { total })
  }
}
