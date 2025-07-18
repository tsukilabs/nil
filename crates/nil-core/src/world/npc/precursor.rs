// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::npc::precursor::{self, PrecursorId};
use crate::village::Village;
use crate::world::World;
use itertools::Itertools;
use rand::seq::IndexedRandom;

impl World {
  pub(crate) fn spawn_precursors(&mut self) -> Result<()> {
    self.spawn_precursor_villages(PrecursorId::A)?;
    self.spawn_precursor_villages(PrecursorId::B)?;
    Ok(())
  }

  fn spawn_precursor_villages(&mut self, id: PrecursorId) -> Result<()> {
    let size = self.continent.size();
    let radius = precursor::initial_territory_radius(size);
    let amount = precursor::initial_village_amount(size);
    let coords = self
      .precursor_manager
      .precursor(id)
      .origin()
      .within_distance_inclusive(radius)
      .into_iter()
      .filter(|coord| coord.is_within_continent(size))
      .collect_vec();

    for (idx, coord) in coords
      .choose_multiple(&mut rand::rng(), amount.into())
      .copied()
      .enumerate()
    {
      let field = self.continent.field_mut(coord)?;
      debug_assert!(field.is_empty());

      *field = Village::builder(coord)
        .name(format!("Precursor {id} {idx}"))
        .owner(id)
        .infrastructure(Infrastructure::with_max_level())
        .build()
        .into();
    }

    Ok(())
  }
}
