// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::luck::Luck;
use crate::behavior::r#impl::idle::IdleBehavior;
use crate::behavior::score::BehaviorScore;
use crate::behavior::{Behavior, BehaviorProcessor};
use crate::continent::{Coord, Distance};
use crate::error::Result;
use crate::ethic::EthicPowerAxis;
use crate::infrastructure::building::Building;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::maneuver::{ManeuverKind, ManeuverRequest};
use crate::military::unit::r#impl::axeman::Axeman;
use crate::military::unit::stats::power::AttackPower;
use crate::ruler::Ruler;
use crate::world::World;
use bon::Builder;
use itertools::Itertools;
use nil_util::iter::IterExt;
use std::collections::HashMap;
use std::ops::ControlFlow;
use tap::{Conv, Pipe};

#[derive(Builder, Debug)]
pub struct PlunderBehavior {
  origin: Coord,
}

impl PlunderBehavior {
  const MAX_DISTANCE: Distance = Distance::new(20);
  const MIN_IDLE_POWER: AttackPower = Axeman::STATS.attack() * 100;
}

impl Behavior for PlunderBehavior {
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let ruler = world.continent().owner_of(self.origin)?;
    if !ruler.is_bot() {
      return Ok(BehaviorScore::MIN);
    }

    if world
      .military()
      .idle_armies_at(self.origin)
      .filter(|army| army.is_owned_by(ruler))
      .sum::<AttackPower>()
      .le(&Self::MIN_IDLE_POWER)
    {
      return Ok(BehaviorScore::MIN);
    }

    let Some(ethics) = world.get_ethics(ruler)? else {
      return Ok(BehaviorScore::MIN);
    };

    let score = match ethics.power() {
      EthicPowerAxis::FanaticMilitarist => BehaviorScore::MAX,
      EthicPowerAxis::Militarist => BehaviorScore::new(0.75),
      EthicPowerAxis::Pacifist => BehaviorScore::new(0.25),
      EthicPowerAxis::FanaticPacifist => BehaviorScore::MIN,
    };

    Ok(score)
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let ruler = world.continent().owner_of(self.origin)?;
    let targets = world
      .continent()
      .cities_within(self.origin, Self::MAX_DISTANCE)
      .filter(|city| {
        let owner = city.owner();
        owner != ruler && !owner.is_precursor()
      })
      .collect_vec();

    if targets.is_empty() {
      return Ok(ControlFlow::Break(()));
    }

    let mut behaviors = vec![IdleBehavior.boxed()];

    let attack = world.military().attack_of(ruler.clone());
    let mut defense_cache = HashMap::new();

    for target in targets {
      let coord = target.coord();
      let owner = world.continent().owner_of(coord)?;

      let defense = *defense_cache
        .entry(owner.clone())
        .or_insert_with(|| {
          world
            .military()
            .defense_of(owner.clone())
            .mean()
        });

      if *attack > (defense * 2) {
        let behavior = PlunderTargetBehavior::builder()
          .origin(self.origin)
          .target(coord)
          .build()
          .boxed();

        behaviors.push(behavior);
      }
    }

    drop(defense_cache);

    BehaviorProcessor::new(world, behaviors)
      .take(1)
      .try_each()?;

    Ok(ControlFlow::Break(()))
  }
}

#[derive(Builder, Debug)]
pub struct PlunderTargetBehavior {
  origin: Coord,
  target: Coord,
}

impl PlunderTargetBehavior {
  fn attacker<'a>(&self, world: &'a World) -> Result<&'a Ruler> {
    world.continent().owner_of(self.origin)
  }

  fn attacker_personnel(&self, world: &World) -> Result<ArmyPersonnel> {
    let attacker = self.attacker(world)?;
    world
      .military()
      .idle_armies_at(self.origin)
      .filter(|army| army.is_owned_by(attacker))
      .sum::<ArmyPersonnel>()
      .pipe(Ok)
  }

  fn defender<'a>(&self, world: &'a World) -> Result<&'a Ruler> {
    world.continent().owner_of(self.target)
  }

  fn defender_personnel(&self, world: &World) -> ArmyPersonnel {
    world
      .military()
      .fold_idle_personnel_at(self.target)
  }
}

impl Behavior for PlunderTargetBehavior {
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let attacker_personnel = self.attacker_personnel(world)?;
    if attacker_personnel.is_empty() {
      return Ok(BehaviorScore::MIN);
    }

    let defender_personnel = self.defender_personnel(world);
    let wall = world
      .infrastructure(self.target)?
      .wall()
      .level();

    let result = world.simulate_battle(
      &attacker_personnel.to_vec(),
      &defender_personnel.to_vec(),
      Luck::new(0),
      wall,
    )?;

    if result.winner().is_defender() {
      return Ok(BehaviorScore::MIN);
    }

    let attack = result
      .attacker_surviving_personnel()
      .attack()
      .conv::<f64>();

    let original_attack = result
      .attacker_personnel()
      .attack()
      .conv::<f64>();

    let surviving_ratio = attack / original_attack;

    let mut score = if surviving_ratio < 0.5 {
      BehaviorScore::MIN
    } else {
      BehaviorScore::new(surviving_ratio)
    };

    let attacker = self.attacker(world)?;
    let Some(attacker_ethics) = world.get_ethics(attacker)? else {
      return Ok(BehaviorScore::MIN);
    };

    // Defender may be a player, so no ethics.
    let defender = self.defender(world)?;
    if let Some(defender_ethics) = world.get_ethics(defender)? {
      let attacker_truth_ethics = attacker_ethics.truth();
      let defender_truth_ethics = defender_ethics.truth();

      // Rulers shouldn't attack those who share their truth ethics,
      // unless they're fanatic militarists. In that case, there should
      // be a small chance of them attacking anyway.
      if attacker_truth_ethics.is_same_variant(defender_truth_ethics) {
        if attacker_ethics.is_fanatic_militarist() {
          score *= 0.2;
        } else {
          return Ok(BehaviorScore::MIN);
        }
      }
    }

    Ok(score)
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let attacker_personnel = self.attacker_personnel(world)?;
    let request = ManeuverRequest::builder()
      .kind(ManeuverKind::Attack)
      .origin(self.origin)
      .destination(self.target)
      .personnel(attacker_personnel)
      .build();

    let _id = world.request_maneuver_with_emit(request, false)?;

    Ok(ControlFlow::Break(()))
  }
}
