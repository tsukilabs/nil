// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::ethic::Ethics;
use crate::infrastructure::Infrastructure;
use crate::npc::bot::BotId;
use crate::ruler::Ruler;
use crate::world::World;
use tap::Pipe;

impl World {
  pub fn cheat_get_ethics(&self, ruler: &Ruler) -> Result<Option<Ethics>> {
    bail_if_cheats_are_not_allowed!(self);
    self.ruler(ruler)?.ethics().cloned().pipe(Ok)
  }

  pub fn cheat_set_bot_ethics(&mut self, id: &BotId, ethics: Ethics) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);
    *self.bot_mut(id)?.ethics_mut() = ethics;
    Ok(())
  }

  pub fn cheat_spawn_bot(&mut self, name: &str, infrastructure: Infrastructure) -> Result<BotId> {
    bail_if_cheats_are_not_allowed!(self);
    self.spawn_bot(name, infrastructure)
  }
}
