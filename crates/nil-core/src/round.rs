// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::player::PlayerId;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::num::NonZeroU32;
use strum::EnumIs;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Round {
  id: RoundId,
  phase: Phase,
}

impl Round {
  pub(crate) fn start<I>(&mut self, players: I) -> Result<()>
  where
    I: IntoIterator<Item = PlayerId>,
  {
    if !self.is_idle() {
      return Err(Error::RoundAlreadyStarted);
    }

    self.phase = Phase::Player {
      pending: players.into_iter().collect(),
    };

    Ok(())
  }

  pub(crate) fn next<I>(&mut self, players: I) -> Result<()>
  where
    I: IntoIterator<Item = PlayerId>,
  {
    if self.is_idle() {
      return Err(Error::RoundNotStarted);
    } else if self.has_pending_players() {
      return Err(Error::RoundHasPendingPlayers);
    }

    self.id = self.id.next();
    self.phase = Phase::Player {
      pending: players.into_iter().collect(),
    };

    Ok(())
  }

  #[inline]
  pub fn phase(&self) -> &Phase {
    &self.phase
  }

  pub(crate) fn phase_mut(&mut self) -> &mut Phase {
    &mut self.phase
  }

  fn pending_players(&self) -> Option<&HashSet<PlayerId>> {
    if let Phase::Player { pending } = &self.phase {
      Some(pending)
    } else {
      None
    }
  }

  fn pending_players_mut(&mut self) -> Option<&mut HashSet<PlayerId>> {
    if let Phase::Player { pending } = &mut self.phase {
      Some(pending)
    } else {
      None
    }
  }

  pub(crate) fn end_turn(&mut self, player: &PlayerId) -> bool {
    if let Some(pending) = self.pending_players_mut() {
      pending.remove(player)
    } else {
      false
    }
  }

  pub fn is_idle(&self) -> bool {
    self.phase.is_idle()
  }

  pub fn has_pending_players(&self) -> bool {
    self
      .pending_players()
      .map(|p| !p.is_empty())
      .unwrap_or(false)
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
pub enum Phase {
  /// Jogo ainda não começou.
  #[default]
  Idle,
  /// Esperando que os jogadores façam seus movimentos.
  Player { pending: HashSet<PlayerId> },
}
