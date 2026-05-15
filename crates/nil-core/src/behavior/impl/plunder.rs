// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::Behavior;
use crate::behavior::score::BehaviorScore;
use crate::continent::{Continent, Coord, Distance};
use crate::error::Result;
use crate::ethic::EthicPowerAxis;
use crate::military::unit::Unit;
use crate::military::unit::r#impl::axeman::Axeman;
use crate::military::unit::stats::power::{AttackPower, Power};
use crate::world::World;
use bon::Builder;
use itertools::Itertools;
use std::ops::ControlFlow;

#[derive(Builder, Debug)]
pub struct PlunderBehavior {
  coord: Coord,
}

impl PlunderBehavior {
  const MAX_DISTANCE: Distance = Distance::new(20);
  const MIN_IDLE_POWER: AttackPower = Axeman::STATS.attack() * 100;
}

impl Behavior for PlunderBehavior {
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let ruler = world.continent().owner_of(self.coord)?;

    if !ruler.is_bot() {
      return Ok(BehaviorScore::MIN);
    }

    if world
      .military()
      .idle_armies_at(self.coord)
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
      EthicPowerAxis::FanaticMilitarist => 1.0,
      EthicPowerAxis::Militarist => 0.75,
      EthicPowerAxis::Pacifist => 0.25,
      EthicPowerAxis::FanaticPacifist => 0.0,
    };

    Ok(BehaviorScore::new(score))
  }

  fn behave(&self, _: &mut World) -> Result<ControlFlow<()>> {
    // let distance = Distance::new(continent.size().get() / 10).min(Self::MAX_DISTANCE);
    // let targets = continent
    //   .cities_within(self.coord, distance)
    //   .filter(|city| {
    //     let owner = city.owner();
    //     owner != ruler && !owner.is_precursor()
    //   })
    //   .collect_vec();

    // if targets.is_empty() {
    //   return Ok(ControlFlow::Break(()));
    // }

    Ok(ControlFlow::Break(()))
  }
}
