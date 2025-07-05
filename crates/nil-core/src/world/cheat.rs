// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::resource::{Food, Iron, Resources, Stone, Wood};
use crate::village::{Coord, Stability};

impl World {
  pub fn cheat_set_resources(&mut self, player_id: PlayerId, resources: Resources) -> Result<()> {
    if self.config.allow_cheats {
      let player = self.player_mut(&player_id)?;
      *player.resources_mut() = resources;
      self.emit_player_resources_updated(player_id);
      Ok(())
    } else {
      Err(Error::CheatingNotAllowed)
    }
  }

  #[inline]
  pub fn cheat_set_max_resources(&mut self, player_id: PlayerId) -> Result<()> {
    self.cheat_set_resources(player_id, Resources::MAX.clone())
  }

  pub fn cheat_set_food(&mut self, player_id: PlayerId, food: Food) -> Result<()> {
    let resources = self
      .player(&player_id)?
      .resources()
      .with_food(food);

    self.cheat_set_resources(player_id, resources)
  }

  pub fn cheat_set_iron(&mut self, player_id: PlayerId, iron: Iron) -> Result<()> {
    let resources = self
      .player(&player_id)?
      .resources()
      .with_iron(iron);

    self.cheat_set_resources(player_id, resources)
  }

  pub fn cheat_set_stone(&mut self, player_id: PlayerId, stone: Stone) -> Result<()> {
    let resources = self
      .player(&player_id)?
      .resources()
      .with_stone(stone);

    self.cheat_set_resources(player_id, resources)
  }

  pub fn cheat_set_wood(&mut self, player_id: PlayerId, wood: Wood) -> Result<()> {
    let resources = self
      .player(&player_id)?
      .resources()
      .with_wood(wood);

    self.cheat_set_resources(player_id, resources)
  }

  pub fn cheat_set_stability(&mut self, coord: Coord, stability: Stability) -> Result<()> {
    if self.config.allow_cheats {
      let village = self.village_mut(coord)?;
      *village.stability_mut() = stability;
      self.emit_village_stability_updated(coord);
      Ok(())
    } else {
      Err(Error::CheatingNotAllowed)
    }
  }
}
