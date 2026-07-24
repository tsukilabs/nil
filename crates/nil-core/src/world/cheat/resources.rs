// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::resources::prelude::*;
use crate::ruler::Ruler;
use crate::world::World;
use tap::Pipe;

pub fn get_resources(world: &World, ruler: &Ruler) -> Result<Resources> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .ruler(ruler)
    .map(|ruler| ruler.resources().clone())
}

pub fn set_resources(world: &mut World, ruler: &Ruler, resources: Resources) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);

  let mut ruler_ref = world.ruler_mut(ruler)?;
  *ruler_ref.resources_mut() = resources;

  if let Some(player) = ruler.player().cloned() {
    world.emit_player(player)?;
  }

  Ok(())
}

#[inline]
pub fn set_max_resources(world: &mut World, ruler: &Ruler) -> Result<()> {
  set_resources(world, ruler, Resources::MAX.clone())
}

pub fn set_max_silo_resources(world: &mut World, ruler: &Ruler) -> Result<()> {
  let resources = Resources::builder().food(Food::MAX).build();
  set_resources(world, ruler, resources)
}

pub fn set_max_warehouse_resources(world: &mut World, ruler: &Ruler) -> Result<()> {
  Resources::builder()
    .iron(Iron::MAX)
    .stone(Stone::MAX)
    .wood(Wood::MAX)
    .build()
    .pipe(|resources| set_resources(world, ruler, resources))
}

pub fn set_food(world: &mut World, ruler: &Ruler, food: Food) -> Result<()> {
  world
    .ruler(ruler)?
    .resources()
    .with_food(food)
    .pipe(|resources| set_resources(world, ruler, resources))
}

#[inline]
pub fn set_max_food(world: &mut World, ruler: &Ruler) -> Result<()> {
  set_food(world, ruler, Food::MAX)
}

pub fn set_iron(world: &mut World, ruler: &Ruler, iron: Iron) -> Result<()> {
  world
    .ruler(ruler)?
    .resources()
    .with_iron(iron)
    .pipe(|resources| set_resources(world, ruler, resources))
}

#[inline]
pub fn set_max_iron(world: &mut World, ruler: &Ruler) -> Result<()> {
  set_iron(world, ruler, Iron::MAX)
}

pub fn set_stone(world: &mut World, ruler: &Ruler, stone: Stone) -> Result<()> {
  world
    .ruler(ruler)?
    .resources()
    .with_stone(stone)
    .pipe(|resources| set_resources(world, ruler, resources))
}

#[inline]
pub fn set_max_stone(world: &mut World, ruler: &Ruler) -> Result<()> {
  set_stone(world, ruler, Stone::MAX)
}

pub fn set_wood(world: &mut World, ruler: &Ruler, wood: Wood) -> Result<()> {
  world
    .ruler(ruler)?
    .resources()
    .with_wood(wood)
    .pipe(|resources| set_resources(world, ruler, resources))
}

#[inline]
pub fn set_max_wood(world: &mut World, ruler: &Ruler) -> Result<()> {
  set_wood(world, ruler, Wood::MAX)
}
