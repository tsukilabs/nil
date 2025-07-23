// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod coord;
mod field;
mod index;
mod size;

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
use crate::village::Village;
use serde::{Deserialize, Serialize};

pub use coord::{Coord, Distance};
pub use field::{Field, PublicField};
pub use index::{ContinentIndex, ContinentKey};
pub use size::ContinentSize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Continent {
  fields: Box<[Field]>,
  size: ContinentSize,
}

impl Continent {
  pub(crate) fn new(size: u8) -> Self {
    let size = ContinentSize::new(size);
    let capacity = usize::from(size.get()).pow(2);
    let mut fields = Vec::with_capacity(capacity);
    fields.resize_with(capacity, Field::default);

    Self {
      fields: fields.into_boxed_slice(),
      size,
    }
  }

  #[inline]
  pub fn size(&self) -> ContinentSize {
    self.size
  }

  #[inline]
  pub fn radius(&self) -> u8 {
    self.size.get().div_ceil(2)
  }

  #[inline]
  pub fn center(&self) -> Coord {
    Coord::splat(self.radius())
  }

  pub fn field(&self, key: impl ContinentKey) -> Result<&Field> {
    let index = key.into_index(self.size);
    self
      .fields
      .get(index.0)
      .ok_or(Error::IndexOutOfBounds(index))
  }

  pub(crate) fn field_mut(&mut self, key: impl ContinentKey) -> Result<&mut Field> {
    let index = key.into_index(self.size);
    self
      .fields
      .get_mut(index.0)
      .ok_or(Error::IndexOutOfBounds(index))
  }

  pub fn fields(&self) -> impl Iterator<Item = &Field> {
    self.fields.iter()
  }

  fn fields_mut(&mut self) -> impl Iterator<Item = &mut Field> {
    self.fields.iter_mut()
  }

  pub fn enumerate_fields(&self) -> impl Iterator<Item = (ContinentIndex, &Field)> {
    self
      .fields()
      .enumerate()
      .map(|(idx, field)| (ContinentIndex::new(idx), field))
  }

  pub fn village(&self, key: impl ContinentKey) -> Result<&Village> {
    let index = key.into_index(self.size);
    if let Some(village) = self.field(index)?.village() {
      Ok(village)
    } else {
      let coord = index.to_coord(self.size)?;
      Err(Error::VillageNotFound(coord))
    }
  }

  pub fn village_mut(&mut self, key: impl ContinentKey) -> Result<&mut Village> {
    let size = self.size;
    let index = key.into_index(size);
    if let Some(village) = self.field_mut(index)?.village_mut() {
      Ok(village)
    } else {
      let coord = index.to_coord(size)?;
      Err(Error::VillageNotFound(coord))
    }
  }

  pub fn villages(&self) -> impl Iterator<Item = &Village> {
    self.fields().filter_map(Field::village)
  }

  pub fn villages_mut(&mut self) -> impl Iterator<Item = &mut Village> {
    self
      .fields_mut()
      .filter_map(Field::village_mut)
  }

  pub fn player_villages_by<F>(&self, f: F) -> impl Iterator<Item = &Village>
  where
    F: Fn(&PlayerId) -> bool,
  {
    self
      .villages()
      .filter(move |village| village.is_owned_by_player_and(&f))
  }

  pub fn player_coords_by<F>(&self, f: F) -> impl Iterator<Item = Coord>
  where
    F: Fn(&PlayerId) -> bool,
  {
    self
      .player_villages_by(f)
      .map(Village::coord)
  }

  pub fn bot_villages_by<F>(&self, f: F) -> impl Iterator<Item = &Village>
  where
    F: Fn(BotId) -> bool,
  {
    self
      .villages()
      .filter(move |village| village.is_owned_by_bot_and(&f))
  }

  pub fn precursor_villages_by<F>(&self, f: F) -> impl Iterator<Item = &Village>
  where
    F: Fn(PrecursorId) -> bool,
  {
    self
      .villages()
      .filter(move |village| village.is_owned_by_precursor_and(&f))
  }
}

impl Default for Continent {
  fn default() -> Self {
    Self::new(ContinentSize::default().get())
  }
}
