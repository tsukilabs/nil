// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::military::maneuver::ManeuverRequest;
use crate::world::World;

impl World {
  pub fn request_maneuver(&mut self, request: &ManeuverRequest) -> Result<()> {
    // TODO: Emit events.
    self.military.request_maneuver(request)
  }
}
