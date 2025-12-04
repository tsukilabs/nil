// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::military::army::ArmyState;
use crate::military::maneuver::{Maneuver, ManeuverId, ManeuverRequest};
use crate::world::World;

impl World {
  pub fn request_maneuver(&mut self, request: &ManeuverRequest) -> Result<ManeuverId> {
    self
      .military
      .collapse_armies_in(request.origin);

    let ruler_origin = self.city(request.origin)?.owner().clone();
    let Some(army) = self
      .military
      .armies_mut_at(request.origin)
      .iter_mut()
      .find(|army| army.is_idle_and_owned_by(&ruler_origin))
      .filter(|army| army.has_enough_personnel(&request.personnel))
    else {
      return Err(Error::InsufficientUnits);
    };

    let (id, maneuver) = Maneuver::builder()
      .army(army.id())
      .kind(request.kind)
      .origin(request.origin)
      .destination(request.destination)
      .build()?;

    *army.state_mut() = ArmyState::from(id);
    self.military.insert_maneuver(maneuver);

    let sender_player = ruler_origin.player();
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
}
