// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::error::Result;
use crate::npc::bot::BotId;
use crate::world::World;

impl World {
  pub fn cheat_spawn_bot(&mut self, name: &str) -> Result<BotId> {
    bail_cheat_not_allowed!(self);
    self.spawn_bot(name)
  }
}
