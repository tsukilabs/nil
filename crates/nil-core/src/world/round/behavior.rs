// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::index::*;
use crate::behavior::{Behavior, BehaviorProcessor};
use crate::error::Result;
use crate::ruler::Ruler;
use crate::world::World;
use itertools::Itertools;
use nil_util::iter::IterExt;

impl World {
  pub(super) fn process_bot_behavior(&mut self) -> Result<()> {
    let bots = self
      .bots()
      .map(|bot| Ruler::from(bot.id()))
      .collect_vec();

    for bot in bots {
      let mut behaviors = vec![IdleBehavior.boxed()];
      behaviors.extend(build(self, &bot));
      BehaviorProcessor::new(self, behaviors).try_each()?;
    }

    Ok(())
  }

  pub(super) fn process_precursor_behavior(&mut self) -> Result<()> {
    let precursors = self
      .precursors()
      .map(|precursor| Ruler::from(precursor.id()))
      .collect_vec();

    for precursor in precursors {
      let mut behaviors = vec![IdleBehavior.boxed()];
      behaviors.extend(build(self, &precursor));
      BehaviorProcessor::new(self, behaviors).try_each()?;
    }

    Ok(())
  }
}

fn build(world: &World, ruler: &Ruler) -> impl Iterator<Item = Box<dyn Behavior>> {
  world
    .continent()
    .cities_of(ruler.clone())
    .map(move |city| {
      BuildBehavior::builder()
        .coord(city.coord())
        .build()
        .boxed()
    })
}
