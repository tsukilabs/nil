// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::luck::Luck;
use crate::battle::{Battle, BattleWinner, DefensivePower, OffensivePower};
use crate::error::Result;
use crate::infrastructure::building::BuildingLevel;
use crate::infrastructure::stats::InfrastructureStats;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::squad::Squad;
use crate::military::squad::size::SquadSize;
use crate::military::unit::UnitId;
use crate::military::unit::UnitId::*;
use crate::world::config::WorldConfig;
use std::assert_matches;
use std::sync::LazyLock;

static STATS: LazyLock<InfrastructureStats> = LazyLock::new(|| {
  let config = WorldConfig::builder("World").build();
  InfrastructureStats::new(&config)
});

#[test]
fn offensive_power() {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .luck(Luck::new(0))
    .infrastructure_stats(&STATS)
    .build();

  let power = offensive(&battle);
  assert_eq!(power.total, 5250.0);
}

#[test]
fn offensive_power_max_luck() {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .luck(Luck::MAX)
    .infrastructure_stats(&STATS)
    .build();

  let power = offensive(&battle);
  assert_eq!(power.total, 6300.0);
}

#[test]
fn offensive_power_min_luck() {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .luck(Luck::MIN)
    .infrastructure_stats(&STATS)
    .build();

  let power = offensive(&battle);
  assert_eq!(power.total, 4200.0);
}

#[test]
fn offensive_power_cavalry() {
  let attacker = [s(HeavyCavalry, 100)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .luck(Luck::new(0))
    .infrastructure_stats(&STATS)
    .build();

  let power = offensive(&battle);
  assert_eq!(power.total, 15000.0);
}

#[test]
fn offensive_power_mixed() {
  let attacker = [s(HeavyCavalry, 100), s(Pikeman, 500)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .luck(Luck::new(0))
    .infrastructure_stats(&STATS)
    .build();

  let power = offensive(&battle);
  assert_eq!(power.total, 20000.0);
}

#[test]
fn defensive_power() -> Result<()> {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let defender = [s(Pikeman, 100), s(Swordsman, 50)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .luck(Luck::new(0))
    .infrastructure_stats(&STATS)
    .build();

  let power = defensive(&battle)?;
  assert_eq!(power.total, 4000.0);

  Ok(())
}

#[test]
fn defensive_power_with_wall() -> Result<()> {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let defender = [s(Pikeman, 100), s(Swordsman, 50)];
  let wall = STATS.wall().get(BuildingLevel::new(20))?;

  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .luck(Luck::new(0))
    .wall(wall)
    .infrastructure_stats(&STATS)
    .build();

  let power = defensive(&battle)?;
  let attacking_power = offensive(&battle);
  assert_eq!(power.total, 18280.0);
  assert_eq!(attacking_power.total, 5250.0);

  Ok(())
}

#[test]
fn battle_result() -> Result<()> {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let defender = [s(Pikeman, 100), s(Swordsman, 50)];
  let wall = STATS.wall().get(BuildingLevel::new(20))?;

  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .luck(Luck::new(0))
    .wall(wall)
    .infrastructure_stats(&STATS)
    .build();

  let attacker: ArmyPersonnel = attacker.iter().cloned().collect();
  let defender_survivors: ArmyPersonnel = [s(Pikeman, 84), s(Swordsman, 42)]
    .into_iter()
    .collect();

  let result = battle.result()?;
  assert_eq!(result.winner, BattleWinner::Defender);
  assert_eq!(result.attacker_personnel, attacker);
  assert_eq!(
    result.attacker_surviving_personnel,
    ArmyPersonnel::default()
  );
  assert_eq!(result.defender_surviving_personnel, defender_survivors);

  Ok(())
}

#[test]
fn ranged_attack_no_debuff() {
  let attacker = [s(Archer, 3000), s(Axeman, 7000)];
  let defender = [s(Pikeman, 8000), s(Swordsman, 8000)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .luck(Luck::new(0))
    .infrastructure_stats(&STATS)
    .build();

  let attack_power = offensive(&battle);
  assert_eq!(attack_power.total, 370000.0);
}

#[test]
fn no_defenders() -> Result<()> {
  let attacker = [s(Axeman, 8000), s(LightCavalry, 5000)];
  let result = Battle::builder()
    .attacker(&attacker)
    .luck(Luck::new(0))
    .infrastructure_stats(&STATS)
    .build()
    .result()?;

  assert_matches!(result.winner, BattleWinner::Attacker);
  assert!(
    !result
      .attacker_surviving_personnel
      .is_empty()
  );

  Ok(())
}

#[test]
fn downgrade_wall() -> Result<()> {
  let attacker = [s(Axeman, 2000), s(Ram, 500)];
  let defender = [s(Pikeman, 100), s(Swordsman, 100)];
  let wall = STATS.wall().get(BuildingLevel::new(20))?;

  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .luck(Luck::new(0))
    .wall(wall)
    .infrastructure_stats(&STATS)
    .build();

  let result = battle.result()?;
  assert_eq!(result.winner, BattleWinner::Attacker);
  assert_eq!(result.wall_level, BuildingLevel::new(20));
  assert!(result.downgraded_wall_level < 0i8);

  Ok(())
}

fn s(id: UnitId, amount: u32) -> Squad {
  Squad::new(id, SquadSize::new(amount))
}

fn offensive(battle: &Battle<'_>) -> OffensivePower {
  OffensivePower::new(battle.attacker, battle.luck)
}

fn defensive(battle: &Battle<'_>) -> Result<DefensivePower> {
  DefensivePower::new(battle.defender, &offensive(battle), battle.wall, &STATS)
}
