// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::behavior::build::{BUILD_TEMPLATE, BuildStep};
use crate::continent::Coord;
use crate::error::Result;
use crate::world::World;
use itertools::Itertools;
use nil_util::result::WrapOk;

impl World {
  pub fn cheat_get_build_steps(&self, coord: Coord) -> Result<Vec<BuildStep>> {
    let infrastructure = self.infrastructure(coord)?;
    BUILD_TEMPLATE
      .iter()
      .filter(|step| !step.is_done(infrastructure))
      .cloned()
      .collect_vec()
      .wrap_ok()
  }
}
