// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::military::army::{Army, ArmyState};
use crate::military::maneuver::{Maneuver, ManeuverId, ManeuverRequest};
use crate::world::World;

impl World {
  pub fn request_maneuver(&mut self, request: ManeuverRequest) -> Result<ManeuverId> {
    self
      .military
      .collapse_armies_in(request.origin);

    let origin_ruler = self.city(request.origin)?.owner().clone();
    let Some(army) = self
      .military
      .armies_mut_at(request.origin)
      .iter_mut()
      .find(|army| army.is_idle_and_owned_by(&origin_ruler))
      .filter(|army| army.has_enough_personnel(&request.personnel))
    else {
      return Err(Error::InsufficientUnits);
    };

    let Some(remaining) = army
      .personnel()
      .checked_sub(&request.personnel)
    else {
      return Err(Error::InsufficientUnits);
    };

    let army_id = army.id();
    let army_speed = army.speed();
    let army_owner = army.owner().clone();
    *army.personnel_mut() = request.personnel;

    let (id, maneuver) = Maneuver::builder()
      .army(army_id)
      .kind(request.kind)
      .origin(request.origin)
      .destination(request.destination)
      .speed(army_speed)
      .build()?;

    self.military.insert_maneuver(maneuver);

    let army = self.military.army_mut(army_id)?;
    *army.state_mut() = ArmyState::with_maneuver(id);

    // The remaining personnel should only be spawned after updating the state of the original army.
    // Otherwise, both would be collapsed into a single army again in the `spawn` call.
    if !remaining.is_empty() {
      Army::builder()
        .owner(army_owner)
        .personnel(remaining)
        .build()
        .spawn(&mut self.military, request.origin);
    }

    let sender_player = origin_ruler.player();
    let target_player = self.city(request.destination)?.player();

    if let Some(sender_player) = sender_player {
      self.emit_military_updated(sender_player.clone());
    }

    if let Some(target_player) = target_player
      && sender_player.is_none_or(|it| it != &target_player)
    {
      self.emit_military_updated(target_player);
    }

    Ok(id)
  }

  #[inline]
  pub fn collapse_armies(&mut self) {
    self.military.collapse_armies();
  }
}
