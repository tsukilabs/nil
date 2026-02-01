// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::build::BuildBehavior;
use crate::behavior::idle::IdleBehavior;
use crate::behavior::recruit::RecruitBehavior;
use crate::behavior::{Behavior, BehaviorProcessor};
use crate::error::Result;
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use nil_util::iter::IterExt;

impl World {
  pub(super) fn process_npc_behavior(&mut self) -> Result<()> {
    self.process_bot_behavior()?;
    self.process_precursor_behavior()?;
    Ok(())
  }

  fn process_bot_behavior(&mut self) -> Result<()> {
    let bots = self
      .bots()
      .map(|bot| Ruler::from(bot.id()))
      .collect_vec();

    for bot in bots {
      let mut behaviors = vec![IdleBehavior.boxed()];
      behaviors.extend(with_coords(self, &bot));
      BehaviorProcessor::new(self, behaviors).try_each()?;
    }

    Ok(())
  }

  fn process_precursor_behavior(&mut self) -> Result<()> {
    let precursors = self
      .precursors()
      .map(|precursor| Ruler::from(precursor.id()))
      .collect_vec();

    for precursor in precursors {
      let mut behaviors = vec![IdleBehavior.boxed()];
      behaviors.extend(with_coords(self, &precursor));
      BehaviorProcessor::new(self, behaviors).try_each()?;
    }

    Ok(())
  }
}

fn with_coords(world: &World, ruler: &Ruler) -> impl Iterator<Item = Box<dyn Behavior>> {
  world
    .continent()
    .coords_of(ruler.clone())
    .flat_map(|coord| {
      [
        BuildBehavior::builder()
          .coord(coord)
          .build()
          .boxed(),
        RecruitBehavior::builder()
          .coord(coord)
          .build()
          .boxed(),
      ]
    })
}
