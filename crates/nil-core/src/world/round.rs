// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::Result;
use crate::player::{Player, PlayerId};
use crate::resource::prelude::*;
use std::collections::HashMap;

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
    if self.round.phase_mut().end_turn(player) {
      self.emit_round_updated();
    }

    if !self.round.has_pending_players() {
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
    self.process_village_queues();
    Ok(())
  }

  /// Updates all players' resources by increasing them with the amount generated
  /// in the current round and then deducting all maintenance-related costs.
  fn update_player_resources(&mut self) -> Result<()> {
    let stats = self.stats.infrastructure.as_ref();
    let mut diff: HashMap<PlayerId, ResourcesDiff> = HashMap::new();

    for village in self.continent.villages() {
      if let Some(player_id) = village.owner().player().cloned() {
        let resources = diff.entry(player_id).or_default();
        *resources += village.round_production(stats)?;
        resources.food -= village.maintenance(stats)?;
      }
    }

    for (player_id, resources) in diff {
      let capacity = self.get_player_storage_capacity(&player_id)?;
      self
        .player_mut(&player_id)?
        .resources_mut()
        .add_if_within_capacity(&resources, &capacity);
    }

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
