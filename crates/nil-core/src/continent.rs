// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod field;

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::village::{Coord, Village};
use serde::{Deserialize, Serialize};
use std::num::{NonZeroU8, NonZeroUsize};

pub use field::{Field, PublicField};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Continent {
  fields: Vec<Field>,
  size: NonZeroUsize,
}

impl Continent {
  pub const MIN_SIZE: NonZeroU8 = NonZeroU8::new(10).unwrap();
  pub const DEFAULT_SIZE: NonZeroU8 = NonZeroU8::new(100).unwrap();

  pub(crate) fn new(size: u8) -> Self {
    let size = size.max(Self::MIN_SIZE.get());
    let size = unsafe { NonZeroUsize::new_unchecked(size.into()) };
    let capacity = size.get().pow(2);

    let mut fields = Vec::with_capacity(capacity);
    fields.resize_with(capacity, Field::default);
    fields.shrink_to_fit();

    Self { fields, size }
  }

  #[inline]
  pub fn size(&self) -> usize {
    self.size.get()
  }

  #[inline]
  pub fn field(&self, coord: Coord) -> Result<&Field> {
    let index = self.index(coord);
    self
      .fields
      .get(index)
      .ok_or(Error::CoordOutOfBounds(coord))
  }

  #[inline]
  pub(crate) fn field_mut(&mut self, coord: Coord) -> Result<&mut Field> {
    let index = self.index(coord);
    self
      .fields
      .get_mut(index)
      .ok_or(Error::CoordOutOfBounds(coord))
  }

  pub fn fields<C>(&self, coords: C) -> Result<Vec<(Coord, &Field)>>
  where
    C: IntoIterator<Item = Coord>,
  {
    coords
      .into_iter()
      .map(|coord| Ok((coord, self.field(coord)?)))
      .try_collect()
  }

  #[inline]
  pub fn village(&self, coord: Coord) -> Result<&Village> {
    self
      .field(coord)?
      .village()
      .ok_or(Error::VillageNotFound(coord))
  }

  #[inline]
  pub fn village_mut(&mut self, coord: Coord) -> Result<&mut Village> {
    self
      .field_mut(coord)?
      .village_mut()
      .ok_or(Error::VillageNotFound(coord))
  }

  pub fn villages(&self) -> impl Iterator<Item = &Village> {
    self.fields.iter().filter_map(Field::village)
  }

  pub fn villages_mut(&mut self) -> impl Iterator<Item = &mut Village> {
    self
      .fields
      .iter_mut()
      .filter_map(Field::village_mut)
  }

  /// Coleta as coordenadas das aldeias de um determinado jogador.
  pub fn player_coords(&self, player: &PlayerId) -> impl Iterator<Item = Coord> {
    self
      .fields
      .iter()
      .filter_map(Field::village)
      .filter(move |village| village.is_owned_by_player_and(|id| id == player))
      .map(Village::coord)
  }

  /// Determina a posição da coordenada no vetor.
  fn index(&self, coord: Coord) -> usize {
    let size = self.size.get();
    let x = usize::from(coord.x());
    let y = usize::from(coord.y());
    let index = (y * size) + x;

    debug_assert!(x <= size);
    debug_assert!(y <= size);
    debug_assert!(index < self.fields.len());

    index
  }

  /// Determina a coordenada a partir de sua posição no vetor.
  fn coord(&self, index: usize) -> Result<Coord> {
    let x = index % self.size;
    let y = index / self.size;

    Ok(Coord::new(
      u8::try_from(x).map_err(|_| Error::IndexOutOfBounds(index))?,
      u8::try_from(y).map_err(|_| Error::IndexOutOfBounds(index))?,
    ))
  }

  pub(crate) fn find_spawn_point(&self) -> Result<Coord> {
    self
      .fields
      .iter()
      .position(Field::is_empty)
      .map(|index| self.coord(index))
      .ok_or(Error::WorldIsFull)?
  }
}

impl Default for Continent {
  fn default() -> Self {
    Self::new(Self::DEFAULT_SIZE.get())
  }
}
