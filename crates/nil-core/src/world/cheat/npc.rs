// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::error::Result;
use crate::ethic::Ethics;
use crate::npc::bot::BotId;
use crate::ruler::Ruler;
use crate::world::World;
use nil_util::result::WrapOk;

impl World {
  pub fn cheat_get_ethics(&self, ruler: &Ruler) -> Result<Option<Ethics>> {
    bail_cheat_not_allowed!(self);
    self
      .ruler(ruler)?
      .ethics()
      .cloned()
      .wrap_ok()
  }

  pub fn cheat_spawn_bot(&mut self, name: &str) -> Result<BotId> {
    bail_cheat_not_allowed!(self);
    self.spawn_bot(name)
  }
}
