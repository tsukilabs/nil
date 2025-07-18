// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod coord;
mod field;

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::village::Village;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::num::NonZeroU8;

pub use coord::Coord;
pub use field::{Field, PublicField};

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

  pub fn fields(&self) -> impl Iterator<Item = &Field> {
    self.fields.iter()
  }

  pub fn enumerate_fields(&self) -> impl Iterator<Item = (usize, &Field)> {
    self.fields.iter().enumerate()
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

  pub fn player_villages_by<F>(&self, f: F) -> impl Iterator<Item = &Village>
  where
    F: Fn(&PlayerId) -> bool,
  {
    self
      .villages()
      .filter(move |village| village.is_owned_by_player_and(|id| f(id)))
  }

  pub fn player_coords_by<F>(&self, f: F) -> impl Iterator<Item = Coord>
  where
    F: Fn(&PlayerId) -> bool,
  {
    self
      .player_villages_by(f)
      .map(Village::coord)
  }

  /// Searches for a coordinate in the slice, returning its index.
  fn index(&self, coord: Coord) -> usize {
    let size = usize::from(self.size);
    let x = usize::from(coord.x());
    let y = usize::from(coord.y());
    let index = (y * size) + x;

    debug_assert!(x < size);
    debug_assert!(y < size);
    debug_assert!(index < self.fields.len());

    index
  }

  pub(crate) fn coord(&self, index: usize) -> Result<Coord> {
    let size = usize::from(self.size);
    let x = index % size;
    let y = index / size;

    debug_assert!(x < size);
    debug_assert!(y < size);

    Ok(Coord::new(
      u8::try_from(x).map_err(|_| Error::IndexOutOfBounds(index))?,
      u8::try_from(y).map_err(|_| Error::IndexOutOfBounds(index))?,
    ))
  }
}

impl Default for Continent {
  fn default() -> Self {
    Self::new(ContinentSize::default().get())
  }
}

#[derive(Clone, Copy, Debug, Deref, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ContinentSize(NonZeroU8);

impl ContinentSize {
  pub const MIN: ContinentSize = unsafe { Self::new_unchecked(100) };
  pub const MAX: ContinentSize = unsafe { Self::new_unchecked(200) };

  pub fn new(size: u8) -> Self {
    let size = size
      .clamp(Self::MIN.0.get(), Self::MAX.0.get())
      .next_multiple_of(10);

    unsafe { Self::new_unchecked(size) }
  }

  /// # Safety
  ///
  /// The size must be between [`ContinentSize::MIN`] and [`ContinentSize::MAX`].
  ///
  /// This actually will only result in undefined behavior if the size is zero,
  /// however, this fact should never be relied upon.
  #[inline]
  pub const unsafe fn new_unchecked(size: u8) -> Self {
    Self(unsafe { NonZeroU8::new_unchecked(size) })
  }
}

impl Default for ContinentSize {
  fn default() -> Self {
    Self::MIN
  }
}

impl From<ContinentSize> for u8 {
  fn from(size: ContinentSize) -> Self {
    size.0.get()
  }
}

impl From<ContinentSize> for u16 {
  fn from(size: ContinentSize) -> Self {
    u16::from(size.0.get())
  }
}

impl From<ContinentSize> for usize {
  fn from(size: ContinentSize) -> Self {
    usize::from(size.0.get())
  }
}

impl From<ContinentSize> for i16 {
  fn from(size: ContinentSize) -> Self {
    i16::from(size.0.get())
  }
}

impl PartialEq<u8> for ContinentSize {
  fn eq(&self, other: &u8) -> bool {
    self.0.get().eq(other)
  }
}

impl PartialOrd<u8> for ContinentSize {
  fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
    self.0.get().partial_cmp(other)
  }
}
