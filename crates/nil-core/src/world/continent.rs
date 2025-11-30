// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{Coord, Field};
use crate::error::{Error, Result};
use crate::world::World;
use rand::seq::IteratorRandom;

impl World {
  pub(super) fn find_spawn_point(&mut self) -> Result<(Coord, &mut Field)> {
    let size = self.continent.size();
    let pm = &self.precursor_manager;
    let coord = self
      .continent
      .enumerate_fields()
      .filter(|(_, field)| field.is_empty())
      .filter_map(|(idx, _)| idx.to_coord(size).ok())
      .filter(|coord| !pm.is_within_territory(*coord, size))
      .choose_stable(&mut rand::rng())
      .ok_or(Error::WorldIsFull)?;

    Ok((coord, self.continent.field_mut(coord)?))
  }
}
