// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::player::PlayerId;
use derive_more::Deref;
use nil_util::iter::IterExt;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::num::NonZeroU32;
use strum::EnumIs;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Round {
  id: RoundId,
  state: RoundState,
}

impl Round {
  pub(crate) fn start<I>(&mut self, players: I) -> Result<()>
  where
    I: IntoIterator<Item = PlayerId>,
  {
    if let RoundState::Idle = &self.state {
      self.wait(players);
      Ok(())
    } else {
      Err(Error::RoundAlreadyStarted)
    }
  }

  /// Tries to advance to the next round.
  pub(crate) fn next<I>(&mut self, players: I) -> Result<()>
  where
    I: IntoIterator<Item = PlayerId>,
  {
    match &self.state {
      RoundState::Idle => Err(Error::RoundNotStarted),
      RoundState::Waiting { pending, .. } if !pending.is_empty() => {
        Err(Error::RoundHasPendingPlayers)
      }
      RoundState::Waiting { .. } | RoundState::Done => {
        self.id = self.id.next();
        self.wait(players);
        Ok(())
      }
    }
  }

  /// Sets the round state to [`RoundState::Waiting`],
  /// where players are expected to take their turns.
  ///
  /// If `players` is empty, the round will be set to [`RoundState::Done`] instead.
  fn wait(&mut self, players: impl IntoIterator<Item = PlayerId>) {
    let pending = players.into_iter().collect_set();
    if pending.is_empty() {
      self.state = RoundState::Done;
    } else {
      let ready = HashSet::with_capacity(pending.len());
      self.state = RoundState::Waiting { pending, ready };
    }
  }

  pub(crate) fn set_ready(&mut self, player: &PlayerId, is_ready: bool) {
    if let RoundState::Waiting { pending, ready } = &mut self.state {
      if is_ready {
        pending.remove(player);
        ready.insert(player.clone());
      } else {
        ready.remove(player);
        pending.insert(player.clone());
      }

      if pending.is_empty() {
        self.state = RoundState::Done;
      }
    }
  }

  #[inline]
  pub fn is_idle(&self) -> bool {
    self.state.is_idle()
  }

  #[inline]
  pub fn is_done(&self) -> bool {
    self.state.is_done()
  }

  #[inline]
  pub fn is_waiting(&self) -> bool {
    self.state.is_waiting()
  }

  #[inline]
  pub fn is_waiting_player(&self, player: &PlayerId) -> bool {
    if let RoundState::Waiting { pending, ready } = &self.state {
      pending.contains(player) || ready.contains(player)
    } else {
      false
    }
  }

  #[inline]
  pub fn is_player_pending(&self, player: &PlayerId) -> bool {
    if let RoundState::Waiting { pending, .. } = &self.state {
      pending.contains(player)
    } else {
      false
    }
  }

  #[inline]
  pub fn is_player_ready(&self, player: &PlayerId) -> bool {
    if let RoundState::Waiting { ready, .. } = &self.state {
      ready.contains(player)
    } else {
      false
    }
  }

  /// Clones the round, setting its state to [`RoundState::Idle`].
  /// This is useful for saving the game.
  pub(crate) fn to_idle(&self) -> Self {
    let mut round = self.clone();
    round.state = RoundState::Idle;
    round
  }
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct RoundId(NonZeroU32);

impl RoundId {
  #[must_use]
  const fn next(self) -> RoundId {
    Self(self.0.saturating_add(1))
  }
}

impl Default for RoundId {
  fn default() -> Self {
    Self(NonZeroU32::MIN)
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum RoundState {
  /// The game hasn't started yet.
  #[default]
  Idle,

  /// There are players who haven't finished their turn yet.
  Waiting {
    pending: HashSet<PlayerId>,
    ready: HashSet<PlayerId>,
  },

  /// The round is finished.
  Done,
}
