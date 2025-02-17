use crate::PlayerId;
use crate::event::Emitter;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::num::NonZeroU32;

#[derive(Debug)]
pub struct Round {
  id: RoundId,
  pending: HashSet<PlayerId>,
  emitter: Emitter,
}

impl Round {
  pub(crate) fn new(emitter: Emitter) -> Self {
    Self {
      id: Default::default(),
      pending: Default::default(),
      emitter,
    }
  }

  pub fn state(&self) -> RoundState {
    RoundState::from(self)
  }
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
struct RoundId(NonZeroU32);

impl RoundId {
  const fn next(&mut self) {
    self.0 = self.0.saturating_add(1);
  }
}

impl Default for RoundId {
  fn default() -> Self {
    Self(unsafe { NonZeroU32::new_unchecked(1) })
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RoundState {
  id: RoundId,
}

impl From<&Round> for RoundState {
  fn from(round: &Round) -> Self {
    RoundState { id: round.id }
  }
}
