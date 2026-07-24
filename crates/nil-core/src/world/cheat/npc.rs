// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::ethic::Ethics;
use crate::infrastructure::Infrastructure;
use crate::npc::bot::BotId;
use crate::ruler::Ruler;
use crate::world::World;

pub fn get_ethics(world: &World, ruler: &Ruler) -> Result<Option<Ethics>> {
  bail_if_cheats_are_not_allowed!(world);
  world.get_ethics(ruler)
}

pub fn set_bot_ethics(world: &mut World, id: &BotId, ethics: Ethics) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);
  *world.bot_mut(id)?.ethics_mut() = ethics;
  Ok(())
}

pub fn spawn_bot(
  world: &mut World,
  name: impl Into<BotId>,
  infrastructure: Infrastructure,
) -> Result<BotId> {
  bail_if_cheats_are_not_allowed!(world);
  world.spawn_bot(name, infrastructure)
}
