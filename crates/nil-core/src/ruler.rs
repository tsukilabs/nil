// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;

pub trait Ruler {
  fn bot(&self) -> Option<&BotId> {
    None
  }

  fn player(&self) -> Option<&PlayerId> {
    None
  }

  fn precursor(&self) -> Option<PrecursorId> {
    None
  }

  fn is_bot(&self) -> bool {
    self.bot().is_some()
  }

  fn is_player(&self) -> bool {
    self.player().is_some()
  }

  fn is_precursor(&self) -> bool {
    self.precursor().is_some()
  }

  fn is_bot_and<F>(&self, f: F) -> bool
  where
    F: FnOnce(&BotId) -> bool,
  {
    self.bot().is_some_and(f)
  }

  fn is_player_and<F>(&self, f: F) -> bool
  where
    F: FnOnce(&PlayerId) -> bool,
  {
    self.player().is_some_and(f)
  }

  fn is_precursor_and<F>(&self, f: F) -> bool
  where
    F: FnOnce(PrecursorId) -> bool,
  {
    self.precursor().is_some_and(f)
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_from_ruler {
  ($from:ident => $into:ident) => {
    impl From<$from> for $into {
      fn from(id: $from) -> Self {
        match id {
          $from::Bot { id } => $into::Bot { id },
          $from::Player { id } => $into::Player { id },
          $from::Precursor { id } => $into::Precursor { id },
        }
      }
    }
  };
}
