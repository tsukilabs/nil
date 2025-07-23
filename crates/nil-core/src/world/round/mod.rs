// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod resources;

use super::World;
use crate::error::Result;
use crate::player::{Player, PlayerId};

impl World {
  pub fn start_round(&mut self) -> Result<()> {
    if self.round.is_idle() {
      let ids = self
        .player_manager
        .players()
        .filter(|player| !player.is_inactive())
        .map(Player::id);

      self.round.start(ids)?;
      self.emit_round_updated();
    }

    Ok(())
  }

  pub fn end_turn(&mut self, player: &PlayerId) -> Result<()> {
    if self.round.end_turn(player) {
      self.emit_round_updated();
    }

    if self.round.is_done() {
      self.next_round()?;
    }

    Ok(())
  }

  fn next_round(&mut self) -> Result<()> {
    let ids = self
      .player_manager
      .players()
      .filter(|player| player.is_active())
      .map(Player::id);

    self.round.next(ids)?;
    self.prepare_next_round()?;

    self.emit_round_updated();
    self.consume_pending_save()?;

    Ok(())
  }

  fn prepare_next_round(&mut self) -> Result<()> {
    self.update_player_resources()?;
    self.update_bot_resources()?;
    self.update_precursor_resources()?;
    self.process_village_queues();
    Ok(())
  }

  /// Processes the build and recruitment queues for all villages.
  fn process_village_queues(&mut self) {
    for village in self.continent.villages_mut() {
      let infra = village.infrastructure_mut();
      infra.process_prefecture_build_queue();
    }
  }
}
