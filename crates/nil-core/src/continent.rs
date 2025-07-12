// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod field;

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::village::{Coord, Village};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

pub use field::{Field, PublicField};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Continent {
  size: NonZeroU8,
  fields: Vec<Field>,
  spawn_radius: NonZeroU8,
}

impl Continent {
  pub const MIN_SIZE: NonZeroU8 = NonZeroU8::new(10).unwrap();
  pub const MAX_SIZE: NonZeroU8 = NonZeroU8::new(200).unwrap();
  pub const DEFAULT_SIZE: NonZeroU8 = NonZeroU8::new(100).unwrap();

  pub(crate) fn new(size: u8) -> Self {
    let size = size
      .clamp(Self::MIN_SIZE.get(), Self::MAX_SIZE.get())
      .next_multiple_of(2);

    let capacity = usize::from(size).pow(2);
    let mut fields = Vec::with_capacity(capacity);
    fields.resize_with(capacity, Field::default);
    fields.shrink_to_fit();

    Self {
      size: unsafe { NonZeroU8::new_unchecked(size) },
      fields,
      spawn_radius: unsafe { NonZeroU8::new_unchecked(10) },
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

  /// Determina a posição da coordenada no vetor.
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

  pub(crate) fn find_spawn_point(&mut self) -> Result<(Coord, &mut Field)> {
    let mut coords = Vec::new();
    let mut rng = rand::rng();

    let coord = loop {
      if self.spawn_radius.get() > self.radius() {
        return Err(Error::WorldIsFull);
      }

      coords.clear();
      self.collect_within_spawn_radius(&mut coords);

      let amount = coords.len() as f64;

      coords.retain(|c| self.field(*c).is_ok_and(|f| f.is_empty()));

      if (coords.len() as f64 / amount) <= 0.2 {
        self.spawn_radius = self.spawn_radius.saturating_add(1);
      }

      if let Some(coord) = coords.choose(&mut rng) {
        break *coord;
      }
    };

    Ok((coord, self.field_mut(coord)?))
  }

  fn collect_within_spawn_radius(&self, buf: &mut Vec<Coord>) {
    let size = i16::from(self.size());
    let distance = i16::from(self.spawn_radius.get());

    let center = self.center();
    let x0 = i16::from(center.x());
    let y0 = i16::from(center.y());

    for x in (x0 - distance)..=(x0 + distance) {
      if x >= size || x < 0 {
        continue;
      }

      for y in (y0 - distance)..=(y0 + distance) {
        if y >= size || y < 0 {
          continue;
        }

        let absx = (x - x0).abs();
        let absy = (y - y0).abs();
        if absx.max(absy) <= distance
          && let Ok(x) = u8::try_from(x)
          && let Ok(y) = u8::try_from(y)
        {
          buf.push(Coord::new(x, y));
        }
      }
    }
  }
}

impl Default for Continent {
  fn default() -> Self {
    Self::new(Self::DEFAULT_SIZE.get())
  }
}
