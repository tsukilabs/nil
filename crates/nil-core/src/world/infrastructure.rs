// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::{Error, Result};
use crate::infrastructure::building::prefecture::PrefectureBuildOrderOptions;
use std::sync::Arc;

impl World {
  pub fn add_prefecture_build_order(
    &mut self,
    options: &PrefectureBuildOrderOptions,
  ) -> Result<()> {
    let stats = Arc::clone(&self.stats.infrastructure);
    let table = stats.building(options.building)?;

    let player_id = self
      .village(options.coord)?
      .owner()
      .player()
      .cloned();

    let curr_res = if let Some(id) = &player_id {
      Some(self.player(id)?.resources().clone())
    } else {
      None
    };

    let coord = options.coord;
    let order = self
      .village_mut(coord)?
      .infrastructure_mut()
      .add_prefecture_build_order(table, curr_res.as_ref(), options)?
      .clone();

    if let Some(id) = player_id {
      if order.kind().is_construction() {
        let player = self.player_mut(&id)?;
        let resources = player.resources_mut();
        *resources = resources
          .checked_sub(order.resources())
          .ok_or(Error::InsufficientResources)?;

        self.emit_player_resources_updated(id.clone());
      }

      self.emit_prefecture_build_queue_updated(id, coord, order.id(), order.kind());
    }

    Ok(())
  }
}
