// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod r#impl;
pub mod score;

use crate::error::Result;
use crate::world::World;
use r#impl::idle::IdleBehavior;
use rand::seq::{IndexedRandom, SliceRandom};
use score::BehaviorScore;
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::ControlFlow;

pub trait Behavior: Any + Debug {
  fn score(&self, world: &World) -> Result<BehaviorScore>;
  fn behave(&self, world: &mut World) -> Result<ControlFlow<()>>;

  fn boxed(self) -> Box<dyn Behavior>
  where
    Self: Sized + 'static,
  {
    Box::new(self)
  }
}

#[must_use]
pub struct BehaviorProcessor<'a> {
  world: &'a mut World,
  behaviors: Vec<Box<dyn Behavior>>,
  buffer: Vec<(usize, BehaviorScore)>,
  candidates: Vec<(usize, f64)>,
  broken: HashSet<usize>,
}

impl<'a> BehaviorProcessor<'a> {
  pub(crate) fn new(world: &'a mut World, mut behaviors: Vec<Box<dyn Behavior>>) -> Self {
    let buffer = Vec::with_capacity(behaviors.len());
    let candidates = Vec::new();
    let broken = HashSet::new();

    behaviors.shuffle(&mut rand::rng());

    Self {
      world,
      behaviors,
      buffer,
      candidates,
      broken,
    }
  }

  fn is_idle(&self, idx: usize) -> bool {
    (self.behaviors[idx].as_ref() as &dyn Any).is::<IdleBehavior>()
  }
}

impl Iterator for BehaviorProcessor<'_> {
  type Item = Result<()>;

  fn next(&mut self) -> Option<Self::Item> {
    self.buffer.clear();
    self.candidates.clear();

    for (idx, behavior) in self.behaviors.iter().enumerate() {
      if !self.broken.contains(&idx) {
        match behavior.score(self.world) {
          Ok(score) => self.buffer.push((idx, score)),
          Err(err) => return Some(Err(err)),
        }
      }
    }

    self
      .buffer
      .sort_unstable_by_key(|(_, score)| *score);

    let highest = self.buffer.last()?;
    if self.is_idle(highest.0) {
      return None;
    }

    self
      .buffer
      .iter()
      .filter(|(_, score)| highest.1.is_within_range(*score, 0.2))
      .map(|(idx, score)| (*idx, f64::from(*score)))
      .collect_into(&mut self.candidates);

    let idx = self
      .candidates
      .choose(&mut rand::rng())
      .map(|(idx, _)| *idx)
      .filter(|idx| !self.is_idle(*idx))?;

    let behavior = &self.behaviors[idx];
    match behavior.behave(self.world) {
      Ok(cf) => {
        if let ControlFlow::Break(()) = cf {
          self.broken.insert(idx);
        }

        Some(Ok(()))
      }
      Err(err) => Some(Err(err)),
    }
  }
}
