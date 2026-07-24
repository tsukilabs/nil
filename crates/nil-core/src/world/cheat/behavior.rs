// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::behavior::r#impl::build::{BUILD_TEMPLATE, BuildStep};
use crate::continent::index::ContinentKey;
use crate::error::Result;
use crate::world::World;
use itertools::Itertools;
use tap::Pipe;

pub fn get_build_steps(world: &World, key: impl ContinentKey) -> Result<Vec<BuildStep>> {
  bail_if_cheats_are_not_allowed!(world);
  let infrastructure = world.infrastructure(key)?;
  BUILD_TEMPLATE
    .iter()
    .filter(|step| !step.is_done(infrastructure))
    .cloned()
    .collect_vec()
    .pipe(Ok)
}
