// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_cheat_not_allowed;
use crate::error::Result;
use crate::player::PlayerId;
use crate::resources::prelude::*;
use crate::world::World;

impl World {
  pub fn cheat_set_resources(&mut self, player_id: PlayerId, resources: Resources) -> Result<()> {
    bail_cheat_not_allowed!(self);
    let player = self.player_mut(&player_id)?;
    *player.resources_mut() = resources;
    self.emit_player_updated(player_id);
    Ok(())
  }

  #[inline]
  pub fn cheat_set_max_resources(&mut self, player_id: PlayerId) -> Result<()> {
    self.cheat_set_resources(player_id, Resources::MAX.clone())
  }

  pub fn cheat_set_max_silo_resources(&mut self, player_id: PlayerId) -> Result<()> {
    let resources = Resources::builder().food(Food::MAX).build();
    self.cheat_set_resources(player_id, resources)
  }

  pub fn cheat_set_max_warehouse_resources(&mut self, player_id: PlayerId) -> Result<()> {
    let resources = Resources::builder()
      .iron(Iron::MAX)
      .stone(Stone::MAX)
      .wood(Wood::MAX)
      .build();

    self.cheat_set_resources(player_id, resources)
  }

  pub fn cheat_set_food(&mut self, player_id: PlayerId, food: Food) -> Result<()> {
    let resources = self
      .player(&player_id)?
      .resources()
      .with_food(food);

    self.cheat_set_resources(player_id, resources)
  }

  #[inline]
  pub fn cheat_set_max_food(&mut self, player_id: PlayerId) -> Result<()> {
    self.cheat_set_food(player_id, Food::MAX)
  }

  pub fn cheat_set_iron(&mut self, player_id: PlayerId, iron: Iron) -> Result<()> {
    let resources = self
      .player(&player_id)?
      .resources()
      .with_iron(iron);

    self.cheat_set_resources(player_id, resources)
  }

  #[inline]
  pub fn cheat_set_max_iron(&mut self, player_id: PlayerId) -> Result<()> {
    self.cheat_set_iron(player_id, Iron::MAX)
  }

  pub fn cheat_set_stone(&mut self, player_id: PlayerId, stone: Stone) -> Result<()> {
    let resources = self
      .player(&player_id)?
      .resources()
      .with_stone(stone);

    self.cheat_set_resources(player_id, resources)
  }

  #[inline]
  pub fn cheat_set_max_stone(&mut self, player_id: PlayerId) -> Result<()> {
    self.cheat_set_stone(player_id, Stone::MAX)
  }

  pub fn cheat_set_wood(&mut self, player_id: PlayerId, wood: Wood) -> Result<()> {
    let resources = self
      .player(&player_id)?
      .resources()
      .with_wood(wood);

    self.cheat_set_resources(player_id, resources)
  }

  #[inline]
  pub fn cheat_set_max_wood(&mut self, player_id: PlayerId) -> Result<()> {
    self.cheat_set_wood(player_id, Wood::MAX)
  }
}
