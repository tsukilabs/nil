// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::military::maneuver::{ManeuverId, ManeuverRequest};
use crate::world::World;

impl World {
  pub fn request_maneuver(&mut self, request: &ManeuverRequest) -> Result<ManeuverId> {
    let player_a = self.city(request.origin)?.player();
    let player_b = self.city(request.destination)?.player();

    // We should make calls that may fail before requesting the maneuver.
    let id = self.military.request_maneuver(request)?;

    if let Some(player_a) = &player_a {
      self.emit_military_updated(player_a.clone());
    }

    if let Some(player_b) = player_b
      && player_a.is_none_or(|it| it != player_b)
    {
      self.emit_military_updated(player_b);
    }

    Ok(id)
  }
}
