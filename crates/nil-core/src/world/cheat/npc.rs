// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::error::Result;
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::npc::bot::{Bot, BotId};
use crate::npc::precursor::PrecursorId;
use crate::resources::Resources;
use crate::world::World;

impl World {
  pub fn cheat_get_bot_resources(&self, id: &BotId) -> Result<Resources> {
    bail_cheat_not_allowed!(self);
    self
      .bot_manager
      .bot(id)
      .map(Bot::resources)
      .cloned()
  }

  pub fn cheat_get_bot_storage_capacity(&self, id: &BotId) -> Result<OverallStorageCapacity> {
    bail_cheat_not_allowed!(self);
    self.get_bot_storage_capacity(id)
  }

  pub fn cheat_get_precursor_resources(&self, id: PrecursorId) -> Result<Resources> {
    bail_cheat_not_allowed!(self);
    let resources = self
      .precursor_manager
      .precursor(id)
      .resources()
      .clone();

    Ok(resources)
  }

  pub fn cheat_get_precursor_storage_capacity(
    &self,
    id: PrecursorId,
  ) -> Result<OverallStorageCapacity> {
    bail_cheat_not_allowed!(self);
    self.get_precursor_storage_capacity(id)
  }

  pub fn cheat_spawn_bot(&mut self, name: &str) -> Result<BotId> {
    bail_cheat_not_allowed!(self);
    self.spawn_bot(name)
  }
}
