// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod coord;
mod field;
mod index;
mod size;

#[cfg(test)]
mod tests;

use crate::city::City;
use crate::error::{Error, Result};
use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
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
      .get(index.as_usize())
      .ok_or(Error::IndexOutOfBounds(index))
  }

  pub(crate) fn field_mut(&mut self, key: impl ContinentKey) -> Result<&mut Field> {
    let index = key.into_index(self.size);
    self
      .fields
      .get_mut(index.as_usize())
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

  pub fn city(&self, key: impl ContinentKey) -> Result<&City> {
    let index = key.into_index(self.size);
    if let Some(city) = self.field(index)?.city() {
      Ok(city)
    } else {
      let coord = index.to_coord(self.size)?;
      Err(Error::CityNotFound(coord))
    }
  }

  pub fn city_mut(&mut self, key: impl ContinentKey) -> Result<&mut City> {
    let size = self.size;
    let index = key.into_index(size);
    if let Some(city) = self.field_mut(index)?.city_mut() {
      Ok(city)
    } else {
      let coord = index.to_coord(size)?;
      Err(Error::CityNotFound(coord))
    }
  }

  pub fn cities(&self) -> impl Iterator<Item = &City> {
    self.fields().filter_map(Field::city)
  }

  pub fn cities_mut(&mut self) -> impl Iterator<Item = &mut City> {
    self.fields_mut().filter_map(Field::city_mut)
  }

  pub fn cities_by<F>(&self, f: F) -> impl Iterator<Item = &City>
  where
    F: Fn(&City) -> bool,
  {
    self.cities().filter(move |city| f(city))
  }

  pub fn player_cities_by<F>(&self, f: F) -> impl Iterator<Item = &City>
  where
    F: Fn(&PlayerId) -> bool,
  {
    self.cities_by(move |city| city.is_owned_by_player_and(&f))
  }

  pub fn player_coords_by<F>(&self, f: F) -> impl Iterator<Item = Coord>
  where
    F: Fn(&PlayerId) -> bool,
  {
    self.player_cities_by(f).map(City::coord)
  }

  pub fn bot_cities_by<F>(&self, f: F) -> impl Iterator<Item = &City>
  where
    F: Fn(BotId) -> bool,
  {
    self.cities_by(move |city| city.is_owned_by_bot_and(&f))
  }

  pub fn bot_coords_by<F>(&self, f: F) -> impl Iterator<Item = Coord>
  where
    F: Fn(BotId) -> bool,
  {
    self.bot_cities_by(f).map(City::coord)
  }

  pub fn precursor_cities_by<F>(&self, f: F) -> impl Iterator<Item = &City>
  where
    F: Fn(PrecursorId) -> bool,
  {
    self.cities_by(move |city| city.is_owned_by_precursor_and(&f))
  }

  pub fn precursor_coords_by<F>(&self, f: F) -> impl Iterator<Item = Coord>
  where
    F: Fn(PrecursorId) -> bool,
  {
    self.precursor_cities_by(f).map(City::coord)
  }
}

impl Default for Continent {
  fn default() -> Self {
    Self::new(ContinentSize::default().get())
  }
}
