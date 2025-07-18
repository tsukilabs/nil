// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod field;

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::village::{Coord, Village};
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

pub use field::{Field, PublicField};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Continent {
  fields: Box<[Field]>,
  size: NonZeroU8,
}

impl Continent {
  pub const MIN_SIZE: NonZeroU8 = NonZeroU8::new(100).unwrap();
  pub const MAX_SIZE: NonZeroU8 = NonZeroU8::new(200).unwrap();

  pub(crate) fn new(size: u8) -> Self {
    let size = size
      .clamp(Self::MIN_SIZE.get(), Self::MAX_SIZE.get())
      .next_multiple_of(10);

    let capacity = usize::from(size).pow(2);
    let mut fields = Vec::with_capacity(capacity);
    fields.resize_with(capacity, Field::default);

    Self {
      fields: fields.into_boxed_slice(),
      size: unsafe { NonZeroU8::new_unchecked(size) },
    }
  }

  #[inline]
  pub fn size(&self) -> u8 {
    self.size.get()
  }

  #[inline]
  pub fn radius(&self) -> u8 {
    self.size().div_ceil(2)
  }

  #[inline]
  pub fn center(&self) -> Coord {
    Coord::splat(self.radius())
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

  pub fn player_coords_by<F>(&self, f: F) -> impl Iterator<Item = Coord>
  where
    F: Fn(&PlayerId) -> bool,
  {
    self
      .player_villages_by(f)
      .map(Village::coord)
  }

  pub fn player_villages_by<F>(&self, f: F) -> impl Iterator<Item = &Village>
  where
    F: Fn(&PlayerId) -> bool,
  {
    self
      .villages()
      .filter(move |village| village.is_owned_by_player_and(|id| f(id)))
  }

  /// Searches for a coordinate in the slice, returning its index.
  fn index(&self, coord: Coord) -> usize {
    let size = usize::from(self.size());
    let x = usize::from(coord.x());
    let y = usize::from(coord.y());
    let index = (y * size) + x;

    debug_assert!(x < size);
    debug_assert!(y < size);
    debug_assert!(index < self.fields.len());

    index
  }

  fn coord(&self, index: usize) -> Result<Coord> {
    let size = usize::from(self.size());
    let x = index % size;
    let y = index / size;

    debug_assert!(x < size);
    debug_assert!(y < size);

    Ok(Coord::new(
      u8::try_from(x).map_err(|_| Error::IndexOutOfBounds(index))?,
      u8::try_from(y).map_err(|_| Error::IndexOutOfBounds(index))?,
    ))
  }

  pub(crate) fn find_spawn_point(&mut self) -> Result<(Coord, &mut Field)> {
    let coord = self
      .fields
      .iter()
      .enumerate()
      .filter(|(_, field)| field.is_empty())
      .choose_stable(&mut rand::rng())
      .map(|(idx, _)| self.coord(idx))
      .ok_or(Error::WorldIsFull)??;

    Ok((coord, self.field_mut(coord)?))
  }
}

impl Default for Continent {
  fn default() -> Self {
    Self::new(Self::MIN_SIZE.get())
  }
}
