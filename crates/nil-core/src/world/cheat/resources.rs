// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::resources::prelude::*;
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn cheat_get_resources(&self, ruler: &Ruler) -> Result<Resources> {
    bail_if_cheats_are_not_allowed!(self);
    self
      .ruler(ruler)
      .map(|ruler| ruler.resources().clone())
  }

  pub fn cheat_set_resources(&mut self, ruler: &Ruler, resources: Resources) -> Result<()> {
    bail_if_cheats_are_not_allowed!(self);

    let mut ruler_ref = self.ruler_mut(ruler)?;
    *ruler_ref.resources_mut() = resources;

    if let Some(player) = ruler.player().cloned() {
      self.emit_player_updated(player);
    }

    Ok(())
  }

  #[inline]
  pub fn cheat_set_max_resources(&mut self, ruler: &Ruler) -> Result<()> {
    self.cheat_set_resources(ruler, Resources::MAX.clone())
  }

  pub fn cheat_set_max_silo_resources(&mut self, ruler: &Ruler) -> Result<()> {
    let resources = Resources::builder().food(Food::MAX).build();
    self.cheat_set_resources(ruler, resources)
  }

  pub fn cheat_set_max_warehouse_resources(&mut self, ruler: &Ruler) -> Result<()> {
    let resources = Resources::builder()
      .iron(Iron::MAX)
      .stone(Stone::MAX)
      .wood(Wood::MAX)
      .build();

    self.cheat_set_resources(ruler, resources)
  }

  pub fn cheat_set_food(&mut self, ruler: &Ruler, food: Food) -> Result<()> {
    let resources = self
      .ruler(ruler)?
      .resources()
      .with_food(food);

    self.cheat_set_resources(ruler, resources)
  }

  #[inline]
  pub fn cheat_set_max_food(&mut self, ruler: &Ruler) -> Result<()> {
    self.cheat_set_food(ruler, Food::MAX)
  }

  pub fn cheat_set_iron(&mut self, ruler: &Ruler, iron: Iron) -> Result<()> {
    let resources = self
      .ruler(ruler)?
      .resources()
      .with_iron(iron);

    self.cheat_set_resources(ruler, resources)
  }

  #[inline]
  pub fn cheat_set_max_iron(&mut self, ruler: &Ruler) -> Result<()> {
    self.cheat_set_iron(ruler, Iron::MAX)
  }

  pub fn cheat_set_stone(&mut self, ruler: &Ruler, stone: Stone) -> Result<()> {
    let resources = self
      .ruler(ruler)?
      .resources()
      .with_stone(stone);

    self.cheat_set_resources(ruler, resources)
  }

  #[inline]
  pub fn cheat_set_max_stone(&mut self, ruler: &Ruler) -> Result<()> {
    self.cheat_set_stone(ruler, Stone::MAX)
  }

  pub fn cheat_set_wood(&mut self, ruler: &Ruler, wood: Wood) -> Result<()> {
    let resources = self
      .ruler(ruler)?
      .resources()
      .with_wood(wood);

    self.cheat_set_resources(ruler, resources)
  }

  #[inline]
  pub fn cheat_set_max_wood(&mut self, ruler: &Ruler) -> Result<()> {
    self.cheat_set_wood(ruler, Wood::MAX)
  }
}
