// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod academy;
mod prefecture;
mod stable;
mod workshop;

#[doc(hidden)]
#[macro_export]
macro_rules! decl_world_recruit_order_fn {
  ($building:ident) => {
    paste::paste! {
      impl World {
        pub fn [<add_ $building:snake _recruit_order>](
          &mut self,
          req: &[<$building RecruitOrderRequest>]
        ) -> Result<()> {
          let player_id = self.city(req.coord)?.player();
          let curr_res = if let Some(id) = &player_id {
            Some(self.player(id)?.resources().clone())
          } else {
            None
          };

          let order = self
            .city_mut(req.coord)?
            .infrastructure_mut()
            .[<add_ $building:snake _recruit_order>](req, curr_res.as_ref())?
            .clone();

          if let Some(id) = player_id {
            let player = self.player_mut(&id)?;
            let resources = player.resources_mut();
            *resources = resources
              .checked_sub(order.resources())
              .ok_or(Error::InsufficientResources)?;

            self.emit_player_updated(id);
            self.emit_city_updated(req.coord);
          }

          Ok(())
        }

        pub fn [<cancel_ $building:snake _recruit_order>](
          &mut self,
          coord: Coord,
          id: [<$building RecruitOrderId>]
        ) -> Result<()> {
          let city = self.city_mut(coord)?;
          if let Some(order) = city.infrastructure_mut().[<cancel_ $building:snake _recruit_order>](id)
            && let Some(id) = city.player()
          {
            let player = self.player_mut(&id)?;
            let resources = player.resources_mut();
            *resources += order.resources();

            self.emit_player_updated(id);
            self.emit_city_updated(coord);
          }

          Ok(())
        }
      }
    }
  };
}
