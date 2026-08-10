// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use tap::Pipe;

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::resources::influence::Influence;
use crate::ruler::Ruler;
use crate::world::World;

pub fn get_influence(world: &World, ruler: &Ruler) -> Result<Influence> {
  bail_if_cheats_are_not_allowed!(world);
  world.ruler(ruler)?.influence().pipe(Ok)
}

pub fn set_influence(world: &mut World, ruler: &Ruler, influence: Influence) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);
  let mut ruler_ref = world.ruler_mut(ruler)?;
  *ruler_ref.influence_mut() = influence;
  world.emit_ruler(ruler)?;

  Ok(())
}
