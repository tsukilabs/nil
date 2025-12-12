// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::idle::IdleBehavior;
use crate::behavior::{Behavior, BehaviorProcessor, BehaviorScore};
use crate::continent::Coord;
use crate::error::Result;
use crate::ethic::EthicPowerAxis;
use crate::infrastructure::building::prefecture::{
  PrefectureBuildOrderKind,
  PrefectureBuildOrderRequest,
};
use crate::infrastructure::prelude::*;
use crate::military::maneuver::Maneuver;
use crate::world::World;
use bon::Builder;
use nil_util::iter::IterExt;
use rand::random_range;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::sync::LazyLock;
use strum::IntoEnumIterator;

pub(crate) static TEMPLATE: LazyLock<Vec<BuildStep>> = LazyLock::new(generate_template);

#[derive(Builder, Debug)]
pub struct BuildBehavior {
  coord: Coord,
}

impl BuildBehavior {
  const MAX_IN_QUEUE: f64 = 3.0;
}

impl Behavior for BuildBehavior {
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let infrastructure = world.infrastructure(self.coord)?;
    let in_queue = infrastructure.prefecture().turns_in_queue();
    let score = BehaviorScore::new(1.0 - (in_queue / Self::MAX_IN_QUEUE));
    Ok(score)
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let mut behaviors = vec![IdleBehavior.boxed()];
    macro_rules! push {
      ($building:ident, $id:expr) => {{
        let behavior = BuildBuildingBehavior::builder()
          .marker(PhantomData::<$building>)
          .coord(self.coord)
          .building($id)
          .build()
          .boxed();

        behaviors.push(behavior);
      }};
    }

    for id in BuildingId::iter() {
      match id {
        BuildingId::Academy => push!(Academy, id),
        BuildingId::Farm => push!(Farm, id),
        BuildingId::IronMine => push!(IronMine, id),
        BuildingId::Prefecture => push!(Prefecture, id),
        BuildingId::Quarry => push!(Quarry, id),
        BuildingId::Sawmill => push!(Sawmill, id),
        BuildingId::Silo => push!(Silo, id),
        BuildingId::Stable => push!(Stable, id),
        BuildingId::Wall => push!(Wall, id),
        BuildingId::Warehouse => push!(Warehouse, id),
      }
    }

    BehaviorProcessor::new(world, behaviors)
      .take(3)
      .try_each()?;

    Ok(ControlFlow::Break(()))
  }
}

#[derive(Builder, Debug)]
pub struct BuildBuildingBehavior<T>
where
  T: Building + Debug,
{
  coord: Coord,
  building: BuildingId,
  marker: PhantomData<T>,
}

impl<T> Behavior for BuildBuildingBehavior<T>
where
  T: Building + Debug + 'static,
{
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let infrastructure = world.infrastructure(self.coord)?;
    let building = infrastructure.building(self.building);

    if !building
      .infrastructure_requirements()
      .has_required_levels(infrastructure)
    {
      return Ok(BehaviorScore::ZERO);
    }

    let level = infrastructure
      .prefecture()
      .resolve_level(self.building, building.level());

    if level >= building.max_level() {
      return Ok(BehaviorScore::ZERO);
    }

    let owner = world.continent().owner_of(self.coord)?;
    let stats = world.stats().infrastructure();
    let required_resources = &stats
      .building(self.building)?
      .get(level + 1u8)?
      .resources;

    if !world
      .ruler(owner)?
      .has_resources(required_resources)
    {
      return Ok(BehaviorScore::ZERO);
    }

    if let BuildingId::Wall = self.building
      && let Some(distance) = world
        .military()
        .maneuvers()
        .filter(|maneuver| maneuver.destination() == self.coord)
        .filter(|maneuver| maneuver.is_attack() && maneuver.is_going())
        .filter_map(Maneuver::pending_distance)
        .min()
    {
      let workforce = stats
        .building(self.building)?
        .get(level + 1u8)
        .map(|it| f64::from(it.workforce))?;

      if workforce <= f64::from(distance) {
        return Ok(BehaviorScore::new(1.0));
      }
    }

    let mut score = if TEMPLATE
      .iter()
      .filter(|step| !step.is_done(infrastructure))
      .take(3)
      .any(|step| step.id == self.building)
    {
      BehaviorScore::new(random_range(0.8..=1.0))
    } else {
      BehaviorScore::ZERO
    };

    let ruler_ref = world.ruler(owner)?;
    if let Some(ethics) = ruler_ref.ethics() {
      if self.building.is_civil() {
        score *= match ethics.power() {
          EthicPowerAxis::Militarist => 0.9,
          EthicPowerAxis::FanaticMilitarist => 0.75,
          EthicPowerAxis::Pacifist => 1.1,
          EthicPowerAxis::FanaticPacifist => 1.25,
        }
      } else {
        score *= match ethics.power() {
          EthicPowerAxis::Militarist => 1.1,
          EthicPowerAxis::FanaticMilitarist => 1.25,
          EthicPowerAxis::Pacifist => 0.9,
          EthicPowerAxis::FanaticPacifist => 0.75,
        }
      }
    }

    Ok(score)
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let order = PrefectureBuildOrderRequest {
      coord: self.coord,
      building: self.building,
      kind: PrefectureBuildOrderKind::Construction,
    };

    world.add_prefecture_build_order(&order)?;

    Ok(ControlFlow::Continue(()))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildStep {
  id: BuildingId,
  level: BuildingLevel,
}

impl BuildStep {
  fn new(id: BuildingId, level: BuildingLevel) -> Self {
    Self { id, level }
  }

  pub fn is_done(&self, infrastructure: &Infrastructure) -> bool {
    self.level <= infrastructure.building(self.id).level()
  }
}

macro_rules! step {
  ($id:ident, $level: expr) => {{ BuildStep::new(BuildingId::$id, BuildingLevel::new($level)) }};
}

fn generate_template() -> Vec<BuildStep> {
  vec![
    step!(Sawmill, 8),
    step!(Quarry, 8),
    step!(IronMine, 8),
    step!(Prefecture, 2),
    step!(Sawmill, 10),
    step!(Quarry, 10),
    step!(IronMine, 10),
    step!(Prefecture, 3),
    step!(Academy, 1),
    step!(Farm, 2),
    step!(Wall, 1),
    step!(Warehouse, 2),
    step!(Silo, 2),
    step!(Sawmill, 12),
    step!(Quarry, 12),
    step!(IronMine, 12),
    step!(Wall, 3),
    step!(Academy, 3),
    step!(Silo, 4),
    step!(Farm, 4),
    step!(Prefecture, 5),
    step!(Sawmill, 15),
    step!(Quarry, 15),
    step!(IronMine, 15),
    step!(Academy, 5),
    step!(Wall, 7),
    step!(Warehouse, 10),
    step!(Prefecture, 10),
    step!(Stable, 1),
    step!(Wall, 10),
    step!(Sawmill, 18),
    step!(Quarry, 18),
    step!(IronMine, 18),
    step!(Stable, 2),
    step!(Silo, 6),
    step!(Farm, 6),
    step!(Stable, 3),
    step!(Wall, 15),
    step!(Warehouse, 13),
    step!(Sawmill, 20),
    step!(Quarry, 20),
    step!(IronMine, 20),
    step!(Silo, 10),
    step!(Farm, 10),
    step!(Warehouse, 17),
    step!(Wall, 20),
    step!(Prefecture, 17),
    step!(Stable, 6),
    step!(Academy, 8),
    step!(Sawmill, 25),
    step!(Quarry, 25),
    step!(IronMine, 25),
    step!(Silo, 15),
    step!(Farm, 15),
    step!(Warehouse, 20),
    step!(Stable, 13),
    step!(Academy, 13),
    step!(Prefecture, 25),
    step!(Academy, 20),
    step!(Silo, 20),
    step!(Farm, 20),
    step!(Warehouse, 23),
    step!(Sawmill, 30),
    step!(Quarry, 30),
    step!(IronMine, 30),
    step!(Silo, 25),
    step!(Farm, 25),
    step!(Warehouse, 27),
    step!(Stable, 20),
    step!(Academy, 25),
    step!(Prefecture, 30),
    step!(Farm, 30),
    step!(Silo, 30),
    step!(Warehouse, 30),
  ]
}
