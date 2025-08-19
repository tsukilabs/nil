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

  pub(crate) fn next<I>(&mut self, players: I) -> Result<()>
  where
    I: IntoIterator<Item = PlayerId>,
  {
    match &self.state {
      RoundState::Idle => Err(Error::RoundNotStarted),
      RoundState::Waiting { .. } => Err(Error::RoundHasPendingPlayers),
      RoundState::Done => {
        self.id = self.id.next();
        self.wait(players);
        Ok(())
      }
    }
  }

  fn wait(&mut self, players: impl IntoIterator<Item = PlayerId>) {
    let players = players.into_iter().collect_set();
    if players.is_empty() {
      self.state = RoundState::Done;
    } else {
      self.state = RoundState::Waiting { players };
    }
  }

  pub(crate) fn end_turn(&mut self, player: &PlayerId) {
    if let RoundState::Waiting { players } = &mut self.state {
      players.remove(player);
      if players.is_empty() {
        self.state = RoundState::Done;
      }
    }
  }

  #[inline]
  pub const fn is_idle(&self) -> bool {
    self.state.is_idle()
  }

  #[inline]
  pub const fn is_waiting(&self) -> bool {
    self.state.is_waiting()
  }

  /// Determines if the round is waiting for a player to complete their turn.
  #[inline]
  pub fn is_waiting_player(&self, player: &PlayerId) -> bool {
    if let RoundState::Waiting { players } = &self.state {
      players.contains(player)
    } else {
      false
    }
  }

  #[inline]
  pub const fn is_done(&self) -> bool {
    self.state.is_done()
  }

  /// Clones the round, setting its state to idle. This is useful for saving the game.
  pub(crate) fn to_idle(&self) -> Self {
    let mut round = self.clone();
    round.state = RoundState::Idle;
    round
  }
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct RoundId(NonZeroU32);

impl RoundId {
  #[inline]
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
  Waiting { players: HashSet<PlayerId> },

  /// The round is finished.
  Done,
}
