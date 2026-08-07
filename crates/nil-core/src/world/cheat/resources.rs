// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::resources::prelude::*;
use crate::ruler::Ruler;
use crate::world::World;

pub fn get_resources(world: &World, ruler: &Ruler) -> Result<Resources> {
  bail_if_cheats_are_not_allowed!(world);
  world
    .ruler(ruler)
    .map(|ruler| ruler.resources())
}

pub fn set_resources(world: &mut World, ruler: &Ruler, resources: Resources) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);

  world
    .ruler_mut(ruler)?
    .resources_mut()
    .replace(resources);

  world.emit_ruler(ruler)?;

  Ok(())
}

#[inline]
pub fn set_max_resources(world: &mut World, ruler: &Ruler) -> Result<()> {
  update_resources(world, ruler, |resources| *resources = Resources::MAX)
}

pub fn set_max_silo_resources(world: &mut World, ruler: &Ruler) -> Result<()> {
  update_resources(world, ruler, |resources| {
    resources.food = Food::MAX;
  })
}

pub fn set_max_warehouse_resources(world: &mut World, ruler: &Ruler) -> Result<()> {
  update_resources(world, ruler, |resources| {
    resources.iron = Iron::MAX;
    resources.stone = Stone::MAX;
    resources.wood = Wood::MAX;
  })
}

#[inline]
pub fn set_food(world: &mut World, ruler: &Ruler, food: Food) -> Result<()> {
  update_resources(world, ruler, |resources| resources.food = food)
}

#[inline]
pub fn set_max_food(world: &mut World, ruler: &Ruler) -> Result<()> {
  update_resources(world, ruler, |resources| resources.food = Food::MAX)
}

#[inline]
pub fn set_iron(world: &mut World, ruler: &Ruler, iron: Iron) -> Result<()> {
  update_resources(world, ruler, |resources| resources.iron = iron)
}

#[inline]
pub fn set_max_iron(world: &mut World, ruler: &Ruler) -> Result<()> {
  update_resources(world, ruler, |resources| resources.iron = Iron::MAX)
}

#[inline]
pub fn set_stone(world: &mut World, ruler: &Ruler, stone: Stone) -> Result<()> {
  update_resources(world, ruler, |resources| resources.stone = stone)
}

#[inline]
pub fn set_max_stone(world: &mut World, ruler: &Ruler) -> Result<()> {
  update_resources(world, ruler, |resources| resources.stone = Stone::MAX)
}

#[inline]
pub fn set_wood(world: &mut World, ruler: &Ruler, wood: Wood) -> Result<()> {
  update_resources(world, ruler, |resources| resources.wood = wood)
}

#[inline]
pub fn set_max_wood(world: &mut World, ruler: &Ruler) -> Result<()> {
  update_resources(world, ruler, |resources| resources.wood = Wood::MAX)
}

fn update_resources(
  world: &mut World,
  ruler: &Ruler,
  f: impl FnOnce(&mut Resources),
) -> Result<()> {
  let mut resources = world.ruler(ruler)?.resources();
  f(&mut resources);
  set_resources(world, ruler, resources)
}
