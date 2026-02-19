// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::player::Player;
use crate::world::World;
use itertools::Itertools;
use tap::Pipe;

impl World {
  pub fn cheat_get_players(&self) -> Result<Vec<Player>> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .players()
      .cloned()
      .collect_vec()
      .pipe(Ok)
  }
}
