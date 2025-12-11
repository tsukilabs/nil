// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod index;

use crate::behavior::index::IdleBehavior;
use crate::error::Result;
use crate::world::World;
use derive_more::Into;
use rand::rngs::ThreadRng;
use rand::seq::{IndexedRandom, SliceRandom};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::{Add, ControlFlow, Div, Mul, Sub};

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
  rng: ThreadRng,
}

impl<'a> BehaviorProcessor<'a> {
  pub(crate) fn new(world: &'a mut World, mut behaviors: Vec<Box<dyn Behavior>>) -> Self {
    let buffer = Vec::with_capacity(behaviors.len());
    let candidates = Vec::new();
    let broken = HashSet::new();

    let mut rng = rand::rng();
    behaviors.shuffle(&mut rng);

    Self {
      world,
      behaviors,
      buffer,
      candidates,
      broken,
      rng,
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

    let last = self.buffer.last()?;
    if self.is_idle(last.0) {
      return None;
    }

    self
      .buffer
      .iter()
      .filter(|(_, score)| last.1.is_within_range(*score, 0.2))
      .map(|(idx, score)| (*idx, f64::from(*score)))
      .collect_into(&mut self.candidates);

    let idx = self
      .candidates
      .choose_weighted(&mut self.rng, |(_, score)| *score)
      .ok()
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

#[derive(Clone, Copy, Debug, Into)]
pub struct BehaviorScore(f64);

impl BehaviorScore {
  pub const ZERO: Self = BehaviorScore::new(0.0);

  #[inline]
  pub const fn new(score: f64) -> Self {
    debug_assert!(score.is_finite());
    debug_assert!(!score.is_subnormal());
    Self(score.clamp(0.0, 1.0))
  }

  #[inline]
  pub(crate) fn is_within_range(self, other: BehaviorScore, range: f64) -> bool {
    (self.0 - other.0).abs() < range
  }
}

impl Default for BehaviorScore {
  fn default() -> Self {
    Self(0.0)
  }
}

impl PartialEq for BehaviorScore {
  fn eq(&self, other: &Self) -> bool {
    matches!(self.0.total_cmp(&other.0), Ordering::Equal)
  }
}

impl Eq for BehaviorScore {}

impl PartialOrd for BehaviorScore {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for BehaviorScore {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.total_cmp(&other.0)
  }
}

impl PartialEq<f64> for BehaviorScore {
  fn eq(&self, other: &f64) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<f64> for BehaviorScore {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Add<f64> for BehaviorScore {
  type Output = f64;

  fn add(self, rhs: f64) -> Self::Output {
    self.0 + rhs
  }
}

impl Sub<f64> for BehaviorScore {
  type Output = f64;

  fn sub(self, rhs: f64) -> Self::Output {
    self.0 - rhs
  }
}

impl Mul<f64> for BehaviorScore {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    self.0 * rhs
  }
}

impl Div<f64> for BehaviorScore {
  type Output = f64;

  fn div(self, rhs: f64) -> Self::Output {
    self.0 / rhs
  }
}
