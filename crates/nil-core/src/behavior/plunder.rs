use crate::behavior::{Behavior, BehaviorScore};
use crate::error::Result;
use crate::world::World;
use std::ops::ControlFlow;

#[derive(Debug)]
pub struct PlunderBehavior;

impl PlunderBehavior {
  const SCORE: BehaviorScore = BehaviorScore::new(0.1);
}

impl Behavior for PlunderBehavior {
  fn score(&self, _: &World) -> Result<BehaviorScore> {
    Ok(Self::SCORE)
  }

  fn behave(&self, _: &mut World) -> Result<ControlFlow<()>> {
    Ok(ControlFlow::Break(()))
  }
}
