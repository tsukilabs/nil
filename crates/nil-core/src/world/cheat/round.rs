// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::error::Result;
use crate::player::Player;
use crate::world::World;
use itertools::Itertools;
use std::num::NonZeroU8;

impl World {
  pub fn cheat_skip_round(&mut self, amount: NonZeroU8) -> Result<()> {
    bail_cheat_not_allowed!(self);

    let amount = amount.get();
    let players = self
      .player_manager
      .active_players()
      .map(Player::id)
      .collect_vec();

    for i in 1..=amount {
      for player in &players {
        self.round.set_ready(player, true);
      }

      self.next_round(i == amount)?;
    }

    Ok(())
  }
}
