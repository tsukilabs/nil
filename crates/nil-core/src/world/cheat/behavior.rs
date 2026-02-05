// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::behavior::build::{BUILD_TEMPLATE, BuildStep};
use crate::continent::Coord;
use crate::error::Result;
use crate::world::World;
use itertools::Itertools;
use tap::Pipe;

impl World {
  pub fn cheat_get_build_steps(&self, coord: Coord) -> Result<Vec<BuildStep>> {
    bail_if_cheats_are_not_allowed!(self);
    let infrastructure = self.infrastructure(coord)?;
    BUILD_TEMPLATE
      .iter()
      .filter(|step| !step.is_done(infrastructure))
      .cloned()
      .collect_vec()
      .pipe(Ok)
  }
}
