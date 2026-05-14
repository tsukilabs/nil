// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::Behavior;
use crate::behavior::score::BehaviorScore;
use crate::continent::Coord;
use crate::error::Result;
use crate::world::World;
use bon::Builder;
use std::ops::ControlFlow;

#[derive(Builder, Debug)]
pub struct PlunderBehavior {
  coord: Coord,
}

impl Behavior for PlunderBehavior {
  fn score(&self, _: &World) -> Result<BehaviorScore> {
    Ok(BehaviorScore::default())
  }

  fn behave(&self, _: &mut World) -> Result<ControlFlow<()>> {
    Ok(ControlFlow::Break(()))
  }
}
