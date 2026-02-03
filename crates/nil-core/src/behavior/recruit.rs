// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::idle::IdleBehavior;
use crate::behavior::{Behavior, BehaviorProcessor, BehaviorScore};
use crate::continent::Coord;
use crate::error::Result;
use crate::ethic::EthicPowerAxis;
use crate::infrastructure::building::academy::AcademyRecruitOrderRequest;
use crate::infrastructure::building::stable::StableRecruitOrderRequest;
use crate::military::unit::prelude::*;
use crate::military::unit::{AcademyUnitId, StableUnitId};
use crate::world::World;
use bon::Builder;
use nil_util::iter::IterExt;
use rand::random_range;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::ops::{Add, ControlFlow};
use strum::IntoEnumIterator;

#[derive(Builder, Debug)]
pub struct RecruitBehavior {
  coord: Coord,
}

impl RecruitBehavior {
  const MAX_IN_QUEUE: u8 = 2;
}

impl Behavior for RecruitBehavior {
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let infrastructure = world.infrastructure(self.coord)?;
    let max_in_queue = f64::from(Self::MAX_IN_QUEUE);

    macro_rules! score {
      ($building:ident) => {{
        if let Some(in_queue) = infrastructure
          .$building()
          .turns_in_recruit_queue()
        {
          BehaviorScore::new(1.0 - (in_queue / max_in_queue))
        } else {
          BehaviorScore::MIN
        }
      }};
    }

    let academy = score!(academy);
    let stable = score!(stable);

    Ok(academy.max(stable))
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let mut behaviors = vec![IdleBehavior.boxed()];
    macro_rules! push {
      ($unit:ident, $id:expr) => {{
        let behavior = RecruitUnitBehavior::builder()
          .marker(PhantomData::<$unit>)
          .coord(self.coord)
          .unit($id)
          .build()
          .boxed();

        behaviors.push(behavior);
      }};
    }

    for id in UnitId::iter() {
      match id {
        UnitId::Archer => push!(Archer, id),
        UnitId::Axeman => push!(Axeman, id),
        UnitId::HeavyCavalry => push!(HeavyCavalry, id),
        UnitId::LightCavalry => push!(LightCavalry, id),
        UnitId::Pikeman => push!(Pikeman, id),
        UnitId::Swordsman => push!(Swordsman, id),
      }
    }

    BehaviorProcessor::new(world, behaviors)
      .take(usize::from(Self::MAX_IN_QUEUE))
      .try_each()?;

    Ok(ControlFlow::Break(()))
  }
}

#[derive(Builder, Debug)]
pub struct RecruitUnitBehavior<T>
where
  T: Unit + Debug,
{
  coord: Coord,
  unit: UnitId,
  marker: PhantomData<T>,
}

impl<T> Behavior for RecruitUnitBehavior<T>
where
  T: Unit + Debug + 'static,
{
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let unit_box = UnitBox::from(self.unit);
    let infrastructure = world.infrastructure(self.coord)?;

    if !infrastructure
      .building(unit_box.building())
      .is_enabled()
    {
      return Ok(BehaviorScore::MIN);
    }

    if !unit_box
      .infrastructure_requirements()
      .has_required_levels(infrastructure)
    {
      return Ok(BehaviorScore::MIN);
    }

    let chunk = unit_box.chunk();
    let owner = world.continent().owner_of(self.coord)?;
    let ruler_ref = world.ruler(owner)?;

    if !ruler_ref.has_resources(&chunk.resources()) {
      return Ok(BehaviorScore::MIN);
    }

    if !world
      .get_maintenance_balance(owner.clone())?
      .add(chunk.maintenance() * 5u32)
      .is_sustainable()
    {
      return Ok(BehaviorScore::MIN);
    }

    let mut score = BehaviorScore::new(random_range(0.8..=1.0));

    if let Some(ethics) = ruler_ref.ethics() {
      if unit_box.is_defensive() {
        score *= match ethics.power() {
          EthicPowerAxis::Militarist => 0.75,
          EthicPowerAxis::FanaticMilitarist => 0.5,
          EthicPowerAxis::Pacifist => 1.25,
          EthicPowerAxis::FanaticPacifist => 1.5,
        }
      } else {
        score *= match ethics.power() {
          EthicPowerAxis::Militarist => 1.25,
          EthicPowerAxis::FanaticMilitarist => 1.5,
          EthicPowerAxis::Pacifist => 0.75,
          EthicPowerAxis::FanaticPacifist => 0.5,
        }
      }
    }

    Ok(score)
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    if let Ok(id) = AcademyUnitId::try_from(self.unit) {
      world.add_academy_recruit_order(&AcademyRecruitOrderRequest {
        coord: self.coord,
        unit: id,
        chunks: NonZeroU32::MIN,
      })?;
    } else if let Ok(id) = StableUnitId::try_from(self.unit) {
      world.add_stable_recruit_order(&StableRecruitOrderRequest {
        coord: self.coord,
        unit: id,
        chunks: NonZeroU32::MIN,
      })?;
    }

    Ok(ControlFlow::Continue(()))
  }
}
