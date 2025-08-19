// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
use crate::resources::prelude::*;
use crate::ruler::Ruler;
use crate::world::World;
use std::collections::HashMap;

impl World {
  /// Updates all players' resources by increasing them with the amount generated
  /// in the current round and then deducting all maintenance-related costs.
  pub(super) fn update_player_resources(&mut self) -> Result<()> {
    let stats = self.stats.infrastructure.as_ref();
    let mut diff: HashMap<PlayerId, ResourcesDiff> = HashMap::new();

    for city in self.continent.cities() {
      if let Some(id) = city.owner().player().cloned() {
        let resources = diff.entry(id).or_default();
        *resources += city.round_production(stats)?;
        resources.food -= city.maintenance(stats)?;
      }
    }

    for (id, resources) in diff {
      let capacity = self.get_player_storage_capacity(&id)?;
      self
        .player_mut(&id)?
        .resources_mut()
        .add_within_capacity(&resources, &capacity);
    }

    Ok(())
  }

  pub(super) fn update_bot_resources(&mut self) -> Result<()> {
    let stats = self.stats.infrastructure.as_ref();
    let mut diff: HashMap<BotId, ResourcesDiff> = HashMap::new();

    for city in self.continent.cities() {
      if let Some(id) = city.owner().bot().cloned() {
        let resources = diff.entry(id).or_default();
        *resources += city.round_production(stats)?;
        resources.food -= city.maintenance(stats)?;
      }
    }

    for (id, resources) in diff {
      let capacity = self.get_bot_storage_capacity(&id)?;
      self
        .bot_manager
        .bot_mut(&id)?
        .resources_mut()
        .add_within_capacity(&resources, &capacity);
    }

    Ok(())
  }

  pub(super) fn update_precursor_resources(&mut self) -> Result<()> {
    let stats = self.stats.infrastructure.as_ref();
    let mut diff: HashMap<PrecursorId, ResourcesDiff> = HashMap::new();

    for city in self.continent.cities() {
      if let Some(id) = city.owner().precursor() {
        let resources = diff.entry(id).or_default();
        *resources += city.round_production(stats)?;
        resources.food -= city.maintenance(stats)?;
      }
    }

    for (id, resources) in diff {
      let capacity = self.get_precursor_storage_capacity(id)?;
      self
        .precursor_manager
        .precursor_mut(id)
        .resources_mut()
        .add_within_capacity(&resources, &capacity);
    }

    Ok(())
  }
}
