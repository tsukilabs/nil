// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::error::Result;
use crate::npc::bot::{Bot, BotId};
use crate::npc::precursor::PrecursorId;
use crate::resources::Resources;
use crate::world::World;
use nil_util::result::WrapOk;

impl World {
  pub fn cheat_get_bot_resources(&self, id: &BotId) -> Result<Resources> {
    bail_cheat_not_allowed!(self);
    self
      .bot_manager
      .bot(id)
      .map(Bot::resources)
      .cloned()
  }

  pub fn cheat_get_precursor_resources(&self, id: PrecursorId) -> Result<Resources> {
    bail_cheat_not_allowed!(self);
    self
      .precursor_manager
      .precursor(id)
      .resources()
      .clone()
      .wrap_ok()
  }

  pub fn cheat_spawn_bot(&mut self, name: &str) -> Result<BotId> {
    bail_cheat_not_allowed!(self);
    self.spawn_bot(name)
  }
}
