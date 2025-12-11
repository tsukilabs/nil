// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::index::IdleBehavior;
use crate::behavior::{Behavior, BehaviorProcessor, BehaviorScore};
use crate::continent::Coord;
use crate::error::Result;
use crate::infrastructure::building::prefecture::{
  PrefectureBuildOrderKind,
  PrefectureBuildOrderRequest,
};
use crate::infrastructure::prelude::*;
use crate::world::World;
use bon::Builder;
use nil_util::iter::IterExt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::ControlFlow;
use strum::IntoEnumIterator;

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
      .ruler(owner.clone())?
      .has_resources(required_resources)
    {
      return Ok(BehaviorScore::ZERO);
    }

    Ok(BehaviorScore::new(0.5))
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
