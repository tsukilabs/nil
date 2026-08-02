// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::coord::Coord;
use crate::error::{Error, Result};
use crate::infrastructure::building::r#impl::prefecture::build_queue::PrefectureBuildOrderRequest;
use crate::world::World;
use std::sync::Arc;

impl World {
  pub fn add_prefecture_build_order(&mut self, req: &PrefectureBuildOrderRequest) -> Result<()> {
    let stats = Arc::clone(&self.stats.infrastructure);
    let table = stats.building(req.building)?;

    let ruler = self.city(req.coord)?.owner().clone();
    let available_resources = self.ruler(&ruler)?.resources();

    let order = self
      .city_mut(req.coord)?
      .infrastructure_mut()
      .add_prefecture_build_order(req, table, available_resources)?
      .clone();

    let kind = order.kind();
    if kind.is_construction() {
      let mut ruler_ref = self.ruler_mut(&ruler)?;
      let resources = ruler_ref.resources_mut();
      *resources = resources
        .checked_sub(order.resources())
        .ok_or(Error::InsufficientResources)?;

      self.emit_ruler(&ruler)?;
      self.emit_city(req.coord)?;
    }

    Ok(())
  }

  pub fn cancel_prefecture_build_order(&mut self, coord: Coord) -> Result<()> {
    let city = self.city_mut(coord)?;
    let order = city
      .infrastructure_mut()
      .cancel_prefecture_build_order();

    if let Some(order) = order {
      let kind = order.kind();
      if kind.is_construction() {
        let ruler = city.owner().clone();
        let mut ruler_ref = self.ruler_mut(&ruler)?;
        let resources = ruler_ref.resources_mut();
        *resources += order.resources();

        self.emit_ruler(&ruler)?;
      }

      self.emit_city(coord)?;
    }

    Ok(())
  }
}
