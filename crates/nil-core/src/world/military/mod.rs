// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::military::army::{Army, ArmyState};
use crate::military::maneuver::{Maneuver, ManeuverId, ManeuverRequest};
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn cancel_maneuver(&mut self, id: ManeuverId) -> Result<()> {
    self.military.cancel_maneuver(id)?;
    self.emit_military_to_maneuver_players(id)?;
    Ok(())
  }

  #[inline]
  pub fn collapse_armies(&mut self) {
    self.military.collapse_armies();
  }

  pub fn is_maneuver_army_owned_by<R>(&self, id: ManeuverId, ruler: R) -> Result<bool>
  where
    R: Into<Ruler>,
  {
    let ruler: Ruler = ruler.into();
    let army_id = self.military.maneuver(id)?.army();
    let army_owner = self.military.army(army_id)?.owner();
    Ok(ruler == *army_owner)
  }

  #[inline]
  pub fn request_maneuver(&mut self, request: ManeuverRequest) -> Result<ManeuverId> {
    self.request_maneuver_with_emit(request, true)
  }

  pub(crate) fn request_maneuver_with_emit(
    &mut self,
    request: ManeuverRequest,
    emit: bool,
  ) -> Result<ManeuverId> {
    self
      .military
      .collapse_armies_in(request.origin);

    let Some(army) = self
      .military
      .armies_mut_at(request.origin)
      .iter_mut()
      .find(|army| army.is_idle_and_owned_by(&request.ruler))
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
    let army_owner = army.owner().clone();
    *army.personnel_mut() = request.personnel;

    // We must take the speed of the army only after updating its personnel,
    // because the slowest unit in it may not have been sent as part of the maneuver.
    // Related issue: https://github.com/tsukilabs/nil/issues/267
    let army_speed = army.speed(&self.config);

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

    if emit {
      self.emit_military_to_maneuver_players(id)?;
    }

    Ok(id)
  }
}
