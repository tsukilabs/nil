use crate::error::{Error, Result};
use crate::event::{Emitter, Event};
use crate::player::PlayerId;
use derive_more::Deref;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::num::NonZeroU32;

#[derive(Debug)]
pub struct TurnScheduler {
  turn: Turn,
  players: IndexSet<PlayerId>,
  queue: VecDeque<PlayerId>,
  emitter: Emitter,
}

impl TurnScheduler {
  pub(crate) fn new(emitter: Emitter) -> Self {
    Self {
      turn: Turn::default(),
      players: IndexSet::new(),
      queue: VecDeque::new(),
      emitter,
    }
  }

  pub fn turn(&self) -> Turn {
    self.turn
  }

  pub(crate) fn add_player(&mut self, player: PlayerId) -> bool {
    self.players.insert(player)
  }

  pub fn next_player(&mut self) -> Result<()> {
    if self.players.is_empty() {
      return Err(Error::NoPlayerToSchedule);
    }

    self.update_state();
    self
      .emitter
      .emit(Event::TurnUpdated { turn: self.turn })
  }

  fn update_state(&mut self) {
    match self.turn.state {
      TurnState::Idle => {
        self.queue.clear();
        self
          .queue
          .extend(self.players.iter().copied());

        if let Some(player) = self.queue.pop_front() {
          self.turn.state = TurnState::Waiting { player };
        }
      }
      TurnState::Waiting { .. } => {
        if let Some(next) = self.queue.pop_front() {
          self.turn.state = TurnState::Waiting { player: next };
        } else {
          self.turn.id.next();
          self.turn.state = TurnState::Idle;
          self.update_state();
        }
      }
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Turn {
  id: TurnId,
  state: TurnState,
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
struct TurnId(NonZeroU32);

impl TurnId {
  const fn next(&mut self) {
    self.0 = self.0.saturating_add(1);
  }
}

impl Default for TurnId {
  fn default() -> Self {
    Self(unsafe { NonZeroU32::new_unchecked(1) })
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum TurnState {
  #[default]
  Idle,
  Waiting {
    player: PlayerId,
  },
}
