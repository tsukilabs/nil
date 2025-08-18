// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::city::City;
use crate::error::{Error, Result};
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::military::Military;
use crate::player::{Player, PlayerId, PlayerStatus};
use crate::resources::Maintenance;

impl World {
  pub fn get_player_maintenance(&self, player: &PlayerId) -> Result<Maintenance> {
    self
      .continent
      .player_cities_by(|id| id == player)
      .try_fold(Maintenance::default(), |acc, city| {
        Ok(acc + city.maintenance(&self.stats.infrastructure)?)
      })
  }

  pub fn get_player_military(&self, player: &PlayerId) -> Military {
    let coords = self
      .continent
      .player_coords_by(|id| id == player);

    self.military.intersection(coords)
  }

  pub fn get_player_storage_capacity(&self, player: &PlayerId) -> Result<OverallStorageCapacity> {
    let cities = self
      .continent
      .player_cities_by(|id| id == player);

    self.get_storage_capacity(cities)
  }

  #[inline]
  pub fn has_player(&self, id: &PlayerId) -> bool {
    self.player_manager.has(id)
  }

  pub fn set_player_status(&mut self, id: &PlayerId, status: PlayerStatus) -> Result<()> {
    *self
      .player_manager
      .player_mut(id)?
      .status_mut() = status;

    self.emit_player_updated(id.clone());

    Ok(())
  }

  pub fn spawn_player(&mut self, mut player: Player) -> Result<()> {
    let id = player.id();
    if self.has_player(&id) {
      Err(Error::PlayerAlreadySpawned(id))
    } else {
      let (coord, field) = self.find_spawn_point()?;
      *field = City::builder(coord)
        .name(&*id)
        .owner(&id)
        .build()
        .into();

      *player.status_mut() = PlayerStatus::Active;
      self.player_manager.spawn(player);

      self.emit_public_city_updated(coord);

      Ok(())
    }
  }
}
