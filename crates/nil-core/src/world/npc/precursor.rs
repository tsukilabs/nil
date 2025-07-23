// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::npc::precursor::{self, PrecursorId};
use crate::village::Village;
use crate::world::World;
use nil_num::roman::ToRoman;
use rand::seq::IndexedRandom;

impl World {
  pub(crate) fn get_precursor_storage_capacity(
    &self,
    precursor: PrecursorId,
  ) -> Result<OverallStorageCapacity> {
    let villages = self
      .continent
      .precursor_villages_by(|id| id == precursor);

    self.get_storage_capacity(villages)
  }

  pub(crate) fn spawn_precursors(&mut self) -> Result<()> {
    self.spawn_precursor_villages(PrecursorId::A)?;
    self.spawn_precursor_villages(PrecursorId::B)?;
    Ok(())
  }

  fn spawn_precursor_villages(&mut self, id: PrecursorId) -> Result<()> {
    let size = self.continent.size();
    let radius = precursor::initial_territory_radius(size);
    let amount = precursor::initial_village_amount(size);
    let mut coords = self
      .precursor_manager
      .precursor(id)
      .origin()
      .within_distance_inclusive(radius);

    coords.retain(|coord| coord.is_within_continent(size));

    for (idx, coord) in coords
      .choose_multiple(&mut rand::rng(), amount.into())
      .copied()
      .enumerate()
    {
      let field = self.continent.field_mut(coord)?;
      debug_assert!(field.is_empty());

      // TODO: Our roman numerals implementation only supports numbers between 1 and 3999.
      // Currently, this is not a problem, but it could become one in the future.
      // In that case, we should come up with a good way to handle it.
      let Some(ridx) = idx.saturating_add(1).to_roman() else { continue };

      *field = Village::builder(coord)
        .name(format!("Precursor {id} {ridx}"))
        .owner(id)
        .infrastructure(Infrastructure::with_max_level())
        .build()
        .into();

      let personnel = if idx.is_multiple_of(3) {
        precursor::initial_offensive_personnel()
      } else {
        precursor::initial_defensive_personnel()
      };

      self.military.spawn(coord, id, personnel);
    }

    Ok(())
  }
}
