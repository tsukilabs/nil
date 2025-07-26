// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::infrastructure::building::stable::{StableRecruitOrderId, StableRecruitOrderRequest};
use crate::world::World;

impl World {
  pub fn add_stable_recruit_order(&mut self, req: &StableRecruitOrderRequest) -> Result<()> {
    let player_id = self.village(req.coord)?.player();
    let curr_res = if let Some(id) = &player_id {
      Some(self.player(id)?.resources().clone())
    } else {
      None
    };

    let order = self
      .village_mut(req.coord)?
      .infrastructure_mut()
      .add_stable_recruit_order(req, curr_res.as_ref())?
      .clone();

    if let Some(id) = player_id {
      let player = self.player_mut(&id)?;
      let resources = player.resources_mut();
      *resources = resources
        .checked_sub(order.resources())
        .ok_or(Error::InsufficientResources)?;

      self.emit_player_updated(id);
      self.emit_village_updated(req.coord);
    }

    Ok(())
  }

  pub fn cancel_stable_recruit_order(
    &mut self,
    coord: Coord,
    id: StableRecruitOrderId,
  ) -> Result<()> {
    let village = self.village_mut(coord)?;
    if let Some(order) = village
      .infrastructure_mut()
      .cancel_stable_recruit_order(id)
      && let Some(id) = village.player()
    {
      let player = self.player_mut(&id)?;
      let resources = player.resources_mut();
      *resources += order.resources();

      self.emit_player_updated(id);
      self.emit_village_updated(coord);
    }

    Ok(())
  }
}
