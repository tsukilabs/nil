// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::Result;
use crate::player::{Player, PlayerId};
use crate::resource::ResourcesDiff;
use std::collections::HashMap;

impl World {
  pub fn start_round(&mut self) -> Result<()> {
    let ids = self
      .player_manager
      .players()
      .filter(|player| !player.is_inactive())
      .map(Player::id);

    self.round.start(ids)?;
    self.emit_round_update();
    Ok(())
  }

  pub fn end_turn(&mut self, player: &PlayerId) -> Result<()> {
    if self.round.end_turn(player) {
      self.emit_round_update();
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

    self.emit_round_update();
    self.consume_pending_save()?;

    Ok(())
  }

  fn prepare_next_round(&mut self) -> Result<()> {
    self.update_player_resources()?;
    self.process_village_queues();
    Ok(())
  }

  /// Atualiza os recursos do jogador, aumentando-os de acordo com a produção,
  /// e então deduzindo todos os custos relacionados à manutenção.
  fn update_player_resources(&mut self) -> Result<()> {
    let stats = self.stats.infrastructure.as_ref();
    let mut diff: HashMap<PlayerId, ResourcesDiff> = HashMap::new();

    for village in self.continent.villages() {
      if let Some(player_id) = village.owner().player() {
        let resources = diff.entry(player_id.clone()).or_default();
        *resources += village.round_production(stats)?;
        resources.food -= village.round_maintenance(stats)?;
      }
    }

    for (player_id, resources) in diff {
      let player = self.player_mut(&player_id)?;
      *player.resources_mut() += resources;
    }

    Ok(())
  }

  /// Processa as filas de construção e de recrutamento de todas as aldeias.
  fn process_village_queues(&mut self) {
    for village in self.continent.villages_mut() {
      village.infrastructure_mut().process_queue();
    }
  }
}
