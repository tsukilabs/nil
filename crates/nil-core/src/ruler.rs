// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::npc::bot::{Bot, BotId};
use crate::npc::precursor::{Precursor, PrecursorId};
use crate::player::{Player, PlayerId};
use crate::resources::Resources;
use serde::{Deserialize, Serialize};

#[allow(variant_size_differences)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Ruler {
  Bot { id: BotId },
  Player { id: PlayerId },
  Precursor { id: PrecursorId },
}

impl Ruler {
  #[inline]
  pub fn bot(&self) -> Option<&BotId> {
    if let Self::Bot { id } = self { Some(id) } else { None }
  }

  #[inline]
  pub fn player(&self) -> Option<&PlayerId> {
    if let Self::Player { id } = self { Some(id) } else { None }
  }

  #[inline]
  pub fn precursor(&self) -> Option<PrecursorId> {
    if let Self::Precursor { id } = self { Some(*id) } else { None }
  }

  #[inline]
  pub fn is_bot(&self) -> bool {
    self.bot().is_some()
  }

  #[inline]
  pub fn is_player(&self) -> bool {
    self.player().is_some()
  }

  #[inline]
  pub fn is_precursor(&self) -> bool {
    self.precursor().is_some()
  }

  pub fn is_bot_and<F>(&self, f: F) -> bool
  where
    F: FnOnce(&BotId) -> bool,
  {
    self.bot().is_some_and(f)
  }

  pub fn is_player_and<F>(&self, f: F) -> bool
  where
    F: FnOnce(&PlayerId) -> bool,
  {
    self.player().is_some_and(f)
  }

  pub fn is_precursor_and<F>(&self, f: F) -> bool
  where
    F: FnOnce(PrecursorId) -> bool,
  {
    self.precursor().is_some_and(f)
  }
}

impl From<&Bot> for Ruler {
  fn from(bot: &Bot) -> Self {
    Self::Bot { id: bot.id() }
  }
}

impl From<BotId> for Ruler {
  fn from(id: BotId) -> Self {
    Self::Bot { id }
  }
}

impl From<&BotId> for Ruler {
  fn from(id: &BotId) -> Self {
    Self::Bot { id: id.clone() }
  }
}

impl From<&Player> for Ruler {
  fn from(player: &Player) -> Self {
    Self::Player { id: player.id() }
  }
}

impl From<PlayerId> for Ruler {
  fn from(id: PlayerId) -> Self {
    Self::Player { id }
  }
}

impl From<&PlayerId> for Ruler {
  fn from(id: &PlayerId) -> Self {
    Self::Player { id: id.clone() }
  }
}

impl From<&dyn Precursor> for Ruler {
  fn from(precursor: &dyn Precursor) -> Self {
    Self::Precursor { id: precursor.id() }
  }
}

impl<T: Precursor> From<&T> for Ruler {
  fn from(precursor: &T) -> Self {
    Self::Precursor { id: precursor.id() }
  }
}

impl From<PrecursorId> for Ruler {
  fn from(id: PrecursorId) -> Self {
    Self::Precursor { id }
  }
}

impl From<RulerRef<'_>> for Ruler {
  fn from(ruler: RulerRef<'_>) -> Self {
    match ruler {
      RulerRef::Bot(bot) => Self::Bot { id: bot.id() },
      RulerRef::Player(player) => Self::Player { id: player.id() },
      RulerRef::Precursor(precursor) => Self::Precursor { id: precursor.id() },
    }
  }
}

impl From<RulerRefMut<'_>> for Ruler {
  fn from(ruler: RulerRefMut<'_>) -> Self {
    match ruler {
      RulerRefMut::Bot(bot) => Self::Bot { id: bot.id() },
      RulerRefMut::Player(player) => Self::Player { id: player.id() },
      RulerRefMut::Precursor(precursor) => Self::Precursor { id: precursor.id() },
    }
  }
}

pub enum RulerRef<'a> {
  Bot(&'a Bot),
  Player(&'a Player),
  Precursor(&'a dyn Precursor),
}

impl<'a> RulerRef<'a> {
  pub fn resources(&'a self) -> &'a Resources {
    match self {
      Self::Bot(bot) => bot.resources(),
      Self::Player(player) => player.resources(),
      Self::Precursor(precursor) => precursor.resources(),
    }
  }
}

impl<'a> From<&'a Bot> for RulerRef<'a> {
  fn from(bot: &'a Bot) -> Self {
    Self::Bot(bot)
  }
}

impl<'a> From<&'a Player> for RulerRef<'a> {
  fn from(player: &'a Player) -> Self {
    Self::Player(player)
  }
}

impl<'a> From<&'a dyn Precursor> for RulerRef<'a> {
  fn from(precursor: &'a dyn Precursor) -> Self {
    Self::Precursor(precursor)
  }
}

pub enum RulerRefMut<'a> {
  Bot(&'a mut Bot),
  Player(&'a mut Player),
  Precursor(&'a mut dyn Precursor),
}

impl<'a> RulerRefMut<'a> {
  pub fn resources_mut(&'a mut self) -> &'a mut Resources {
    match self {
      Self::Bot(bot) => bot.resources_mut(),
      Self::Player(player) => player.resources_mut(),
      Self::Precursor(precursor) => precursor.resources_mut(),
    }
  }
}

impl<'a> From<&'a mut Bot> for RulerRefMut<'a> {
  fn from(bot: &'a mut Bot) -> Self {
    Self::Bot(bot)
  }
}

impl<'a> From<&'a mut Player> for RulerRefMut<'a> {
  fn from(player: &'a mut Player) -> Self {
    Self::Player(player)
  }
}

impl<'a> From<&'a mut dyn Precursor> for RulerRefMut<'a> {
  fn from(precursor: &'a mut dyn Precursor) -> Self {
    Self::Precursor(precursor)
  }
}
