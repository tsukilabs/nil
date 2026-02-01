// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::idle::IdleBehavior;
use crate::behavior::{Behavior, BehaviorProcessor, BehaviorScore};
use crate::continent::Coord;
use crate::error::Result;
use crate::world::World;
use bon::Builder;
use nil_util::iter::IterExt;
use std::ops::ControlFlow;

#[derive(Builder, Debug)]
pub struct RecruitBehavior {
  coord: Coord,
}

impl RecruitBehavior {
  const MAX_IN_QUEUE: f64 = 1.0;
}

impl Behavior for RecruitBehavior {
  fn score(&self, world: &World) -> Result<BehaviorScore> {
    let infrastructure = world.infrastructure(self.coord)?;
    let in_academy_queue = infrastructure
      .academy()
      .turns_in_recruit_queue();
    let in_stable_queue = infrastructure
      .stable()
      .turns_in_recruit_queue();

    let in_queue = in_academy_queue.min(in_stable_queue);
    Ok(BehaviorScore::new(1.0 - (in_queue / Self::MAX_IN_QUEUE)))
  }

  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>> {
    let behaviors = vec![IdleBehavior.boxed()];
    BehaviorProcessor::new(world, behaviors)
      .take(1)
      .try_each()?;

    Ok(ControlFlow::Break(()))
  }
}
