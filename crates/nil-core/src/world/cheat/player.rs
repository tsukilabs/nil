// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::player::{Player, PlayerId};
use crate::world::World;
use itertools::Itertools;
use tap::Pipe;

pub fn get_player(world: &World, id: &PlayerId) -> Result<Player> {
  bail_if_cheats_are_not_allowed!(world);
  world.player(id).cloned()
}

pub fn get_players(world: &World) -> Result<Vec<Player>> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .players()
    .cloned()
    .collect_vec()
    .pipe(Ok)
}
