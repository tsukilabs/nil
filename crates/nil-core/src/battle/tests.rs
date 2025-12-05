// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::{Battle, BattleWinner};
use crate::infrastructure::InfrastructureStats;
use crate::infrastructure::building::BuildingLevel;
use crate::military::army::ArmyPersonnel;
use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::UnitId;
use crate::military::unit::UnitId::*;
use std::sync::LazyLock;

static STATS: LazyLock<InfrastructureStats> = LazyLock::new(InfrastructureStats::default);

#[test]
fn offensive_power() {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let battle = Battle::builder().attacker(&attacker).build();

  let power = battle.offensive_power();
  assert_eq!(power.total, 5250.0);
  assert_eq!(power.infantry_ratio, 1.0);
}

#[test]
fn offensive_power_cavalry() {
  let attacker = [s(HeavyCavalry, 100)];
  let battle = Battle::builder().attacker(&attacker).build();

  let power = battle.offensive_power();
  assert_eq!(power.total, 15000.0);
  assert_eq!(power.cavalry_ratio, 1.0);
}

#[test]
fn offensive_power_mixed() {
  let attacker = [s(HeavyCavalry, 100), s(Pikeman, 500)];
  let battle = Battle::builder().attacker(&attacker).build();

  let power = battle.offensive_power();
  assert_eq!(power.total, 20000.0);
  assert_eq!(power.cavalry_ratio, 0.75);
  assert_eq!(power.infantry_ratio, 0.25);
  assert_eq!(power.ranged_ratio, 0.0);
}

#[test]
fn defensive_power() {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let defender = [s(Pikeman, 100), s(Swordsman, 50)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .build();

  let power = battle.defensive_power();
  assert_eq!(power.total, 4000.0);
}

#[test]
fn defensive_power_with_wall() {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let defender = [s(Pikeman, 100), s(Swordsman, 50)];
  let wall = STATS
    .wall()
    .get(BuildingLevel::new(20))
    .unwrap();

  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .wall(wall)
    .build();

  let power = battle.defensive_power();
  let attacking_power = battle.offensive_power();
  assert_eq!(power.total, 18280.0);
  assert_eq!(attacking_power.total, 5250.0);
}

#[test]
fn battle_result() {
  let attacker = [s(Axeman, 100), s(Swordsman, 50)];
  let defender = [s(Pikeman, 100), s(Swordsman, 50)];
  let wall = STATS
    .wall()
    .get(BuildingLevel::new(20))
    .unwrap();

  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .wall(wall)
    .build();

  let attacker: ArmyPersonnel = attacker.iter().cloned().collect();
  let defender_survivors: ArmyPersonnel = [s(Pikeman, 71), s(Swordsman, 35)]
    .into_iter()
    .collect();

  let result = battle.battle_result();
  assert_eq!(result.winner, BattleWinner::Defender);
  assert_eq!(result.attacker_personnel, attacker);
  assert_eq!(
    result.attacker_surviving_personnel,
    ArmyPersonnel::default()
  );
  assert_eq!(result.defender_surviving_personnel, defender_survivors);
}

#[test]
fn ranged_attack_no_debuff() {
  let attacker = [s(Archer, 3000), s(Axeman, 7000)];
  let defender = [s(Pikeman, 8000), s(Swordsman, 8000)];
  let battle = Battle::builder()
    .attacker(&attacker)
    .defender(&defender)
    .build();

  let attack_power = battle.offensive_power();
  assert_eq!(attack_power.total, 370000.0);
}

fn s(id: UnitId, amount: u32) -> Squad {
  Squad::new(id, SquadSize::new(amount))
}
