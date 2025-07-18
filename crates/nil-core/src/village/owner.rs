// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
use serde::{Deserialize, Serialize};

#[expect(variant_size_differences)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum VillageOwner {
  Bot { id: BotId },
  Player { id: PlayerId },
  Precursor { id: PrecursorId },
}

impl VillageOwner {
  /// Returns the id of the bot to whom the village belongs, if any.
  #[inline]
  pub fn bot(&self) -> Option<BotId> {
    if let Self::Bot { id } = self {
      Some(*id)
    } else {
      None
    }
  }

  /// Returns the id of the player to whom the village belongs, if any.
  #[inline]
  pub fn player(&self) -> Option<&PlayerId> {
    if let Self::Player { id } = self {
      Some(id)
    } else {
      None
    }
  }
}

impl From<BotId> for VillageOwner {
  fn from(id: BotId) -> Self {
    Self::Bot { id }
  }
}

impl From<PlayerId> for VillageOwner {
  fn from(id: PlayerId) -> Self {
    Self::Player { id }
  }
}

impl From<&PlayerId> for VillageOwner {
  fn from(id: &PlayerId) -> Self {
    Self::Player { id: id.clone() }
  }
}

impl From<PrecursorId> for VillageOwner {
  fn from(id: PrecursorId) -> Self {
    Self::Precursor { id }
  }
}
