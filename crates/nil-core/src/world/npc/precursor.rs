// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::City;
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::npc::precursor::{self, Precursor, PrecursorId, PrecursorManager};
use crate::world::World;
use nil_num::roman::ToRoman;
use rand::seq::IndexedRandom;

impl World {
  #[inline]
  pub fn precursor_manager(&self) -> &PrecursorManager {
    &self.precursor_manager
  }

  #[inline]
  pub fn precursor(&self, id: PrecursorId) -> &dyn Precursor {
    self.precursor_manager.precursor(id)
  }

  #[inline]
  pub(crate) fn precursor_mut(&mut self, id: PrecursorId) -> &mut dyn Precursor {
    self.precursor_manager.precursor_mut(id)
  }

  pub fn precursors(&self) -> impl Iterator<Item = &dyn Precursor> {
    self.precursor_manager.precursors()
  }

  pub(crate) fn spawn_precursors(&mut self) -> Result<()> {
    self.spawn_precursor_cities(PrecursorId::A)?;
    self.spawn_precursor_cities(PrecursorId::B)?;
    Ok(())
  }

  fn spawn_precursor_cities(&mut self, id: PrecursorId) -> Result<()> {
    let size = self.continent.size();
    let radius = precursor::initial_territory_radius(size);
    let amount = precursor::initial_city_amount(size);
    let mut coords = self
      .precursor_manager
      .precursor(id)
      .origin()
      .within_distance_inclusive(radius);

    coords.retain(|coord| coord.is_within_continent(size));

    for (idx, coord) in coords
      .sample(&mut rand::rng(), amount.into())
      .copied()
      .enumerate()
    {
      let field = self.continent.field_mut(coord)?;
      debug_assert!(field.is_empty());

      // TODO: Our roman numerals implementation only supports numbers between 1 and 3999.
      // Currently, this is not a problem, but it could become one in the future.
      let Some(ridx) = idx.saturating_add(1).to_roman() else { continue };

      *field = City::builder(coord)
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
