// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::military::maneuver::ManeuverRequest;
use crate::world::World;

impl World {
  pub fn request_maneuver(&mut self, request: &ManeuverRequest) -> Result<()> {
    // TODO: Should a player be allowed to attack themselves?
    self.military.request_maneuver(request)?;
    let player_a = self.city(request.origin)?.player();
    let player_b = self.city(request.destination)?.player();

    if let Some(player_a) = &player_a {
      self.emit_military_updated(player_a.clone());
    }

    if player_b != player_a
      && let Some(player_b) = &player_b
    {
      self.emit_military_updated(player_b.clone());
    }

    Ok(())
  }
}
