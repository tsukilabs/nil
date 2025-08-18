// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::infrastructure::building::prefecture::PrefectureBuildOrderRequest;
use crate::world::World;
use std::sync::Arc;

impl World {
  pub fn add_prefecture_build_order(&mut self, req: &PrefectureBuildOrderRequest) -> Result<()> {
    let stats = Arc::clone(&self.stats.infrastructure);
    let table = stats.building(req.building)?;

    let player_id = self.city(req.coord)?.player();
    let curr_res = if let Some(id) = &player_id {
      Some(self.player(id)?.resources().clone())
    } else {
      None
    };

    let order = self
      .city_mut(req.coord)?
      .infrastructure_mut()
      .add_prefecture_build_order(req, table, curr_res.as_ref())?
      .clone();

    if let Some(id) = player_id {
      let kind = order.kind();
      if kind.is_construction() {
        let player = self.player_mut(&id)?;
        let resources = player.resources_mut();
        *resources = resources
          .checked_sub(order.resources())
          .ok_or(Error::InsufficientResources)?;

        self.emit_player_updated(id);
      }

      self.emit_city_updated(req.coord);
    }

    Ok(())
  }

  pub fn cancel_prefecture_build_order(&mut self, coord: Coord) -> Result<()> {
    let city = self.city_mut(coord)?;
    if let Some(order) = city
      .infrastructure_mut()
      .cancel_prefecture_build_order()
      && let Some(id) = city.player()
    {
      let kind = order.kind();
      if kind.is_construction() {
        let player = self.player_mut(&id)?;
        let resources = player.resources_mut();
        *resources += order.resources();

        self.emit_player_updated(id);
      }

      self.emit_city_updated(coord);
    }

    Ok(())
  }
}
