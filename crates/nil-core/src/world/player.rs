// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::{Error, Result};
use crate::player::{Player, PlayerId, PlayerStatus};
use crate::village::{PublicVillage, Village};

impl World {
  #[inline]
  pub fn has_player(&self, id: &PlayerId) -> bool {
    self.player_manager.has(id)
  }

  pub fn remove_guest(&mut self, id: &PlayerId) -> Result<()> {
    if let Some(player) = self.player_manager.remove_guest(id) {
      self.emit_guest_left(player);
    }

    Ok(())
  }

  pub fn set_player_status(&mut self, id: &PlayerId, status: PlayerStatus) -> Result<()> {
    *self
      .player_manager
      .player_mut(id)?
      .status_mut() = status;

    self.emit_player_status_updated(id.clone(), status);

    Ok(())
  }

  pub fn spawn_player(&mut self, player: Player) -> Result<()> {
    let id = player.id();
    if self.has_player(&id) {
      Err(Error::PlayerAlreadySpawned(id))
    } else {
      self.player_manager.insert(player.clone());
      self.emit_player_spawned(player);
      Ok(())
    }
  }

  pub fn spawn_player_village(&mut self, id: PlayerId) -> Result<()> {
    let player = self.player_manager.player_mut(&id)?;
    if player.is_guest() {
      let coord = self.continent.find_spawn_point()?;
      let village = Village::builder(coord)
        .name(&*id)
        .owner(&id)
        .build();

      let public = PublicVillage::from(&village);
      *self.continent.field_mut(coord)? = village.into();
      self.set_player_status(&id, PlayerStatus::Active)?;

      self.emit_village_spawned(public);
      Ok(())
    } else {
      Err(Error::NotAGuest(id))
    }
  }
}
