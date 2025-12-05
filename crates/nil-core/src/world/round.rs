// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::player::{Player, PlayerId};
use crate::resources::prelude::*;
use crate::ruler::Ruler;
use crate::world::World;
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

  pub fn set_player_ready(&mut self, player: &PlayerId, is_ready: bool) -> Result<()> {
    self.round.set_ready(player, is_ready);

    if self.round.is_done() {
      self.next_round()?;
    } else {
      self.emit_round_updated();
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
    self.update_resources()?;
    self.process_city_queues();
    self.update_ranking()?;
    self.military.collapse_armies();
    Ok(())
  }

  /// Updates all rulers' resources by increasing them with the amount generated
  /// in the current round and then deducting all maintenance-related costs.
  fn update_resources(&mut self) -> Result<()> {
    let stats = self.stats.infrastructure.as_ref();
    let mut diff: HashMap<Ruler, ResourcesDiff> = HashMap::new();

    for city in self.continent.cities() {
      let owner = city.owner().clone();
      let resources = diff.entry(owner).or_default();
      *resources += city.round_production(stats)?;
      resources.food -= city.maintenance(stats)?;
    }

    for (ruler, mut resources) in diff {
      resources.food -= self.military.maintenance_of(ruler.clone());
      let capacity = self.get_storage_capacity(ruler.clone())?;
      self
        .ruler_mut(ruler)?
        .resources_mut()
        .add_within_capacity(&resources, &capacity);
    }

    Ok(())
  }

  /// Processes the build and recruitment queues for all cities.
  fn process_city_queues(&mut self) {
    for city in self.continent.cities_mut() {
      let coord = city.coord();
      let owner = city.owner().clone();
      let infra = city.infrastructure_mut();
      infra.process_prefecture_build_queue();

      if let Some(personnel) = infra.process_academy_recruit_queue() {
        self
          .military
          .spawn(coord, owner.clone(), personnel);
      }

      if let Some(personnel) = infra.process_stable_recruit_queue() {
        self
          .military
          .spawn(coord, owner.clone(), personnel);
      }
    }
  }
}
