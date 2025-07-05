// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::village::{Coord, Village};
use serde::{Deserialize, Serialize};
use std::num::{NonZeroU8, NonZeroUsize};
use strum::EnumIs;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Continent {
  cells: Vec<Cell>,
  size: NonZeroUsize,
}

impl Continent {
  pub const MIN_SIZE: NonZeroU8 = NonZeroU8::new(10).unwrap();
  pub const DEFAULT_SIZE: NonZeroU8 = NonZeroU8::new(100).unwrap();

  pub(crate) fn new(size: u8) -> Self {
    let size = size.max(Self::MIN_SIZE.get());
    let size = unsafe { NonZeroUsize::new_unchecked(size.into()) };
    let capacity = size.get().pow(2);

    let mut cells = Vec::with_capacity(capacity);
    cells.resize_with(capacity, Cell::default);
    cells.shrink_to_fit();

    Self { cells, size }
  }

  #[inline]
  pub fn size(&self) -> usize {
    self.size.get()
  }

  #[inline]
  pub fn cell(&self, coord: Coord) -> Result<&Cell> {
    let index = self.index(coord);
    self
      .cells
      .get(index)
      .ok_or(Error::CoordOutOfBounds(coord))
  }

  #[inline]
  pub fn cell_mut(&mut self, coord: Coord) -> Result<&mut Cell> {
    let index = self.index(coord);
    self
      .cells
      .get_mut(index)
      .ok_or(Error::CoordOutOfBounds(coord))
  }

  #[inline]
  pub fn village(&self, coord: Coord) -> Result<&Village> {
    self
      .cell(coord)?
      .village()
      .ok_or(Error::VillageNotFound(coord))
  }

  #[inline]
  pub fn village_mut(&mut self, coord: Coord) -> Result<&mut Village> {
    self
      .cell_mut(coord)?
      .village_mut()
      .ok_or(Error::VillageNotFound(coord))
  }

  pub fn villages(&self) -> impl Iterator<Item = &Village> {
    self.cells.iter().filter_map(Cell::village)
  }

  pub fn villages_mut(&mut self) -> impl Iterator<Item = &mut Village> {
    self
      .cells
      .iter_mut()
      .filter_map(Cell::village_mut)
  }

  /// Coleta as coordenadas das aldeias de um determinado jogador.
  pub fn player_coords(&self, player: &PlayerId) -> impl Iterator<Item = Coord> {
    self
      .cells
      .iter()
      .filter_map(Cell::village)
      .filter(move |village| village.is_owned_by_player_and(|id| id == player))
      .map(Village::coord)
  }

  /// Determina a posição da coordenada no vetor.
  fn index(&self, coord: Coord) -> usize {
    let x = usize::from(coord.x());
    let y = usize::from(coord.y());
    let index = (y * self.size.get()) + x;
    debug_assert!(index < self.cells.len());
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
      .cells
      .iter()
      .position(Cell::is_empty)
      .map(|index| self.coord(index))
      .ok_or(Error::WorldIsFull)?
  }
}

impl Default for Continent {
  fn default() -> Self {
    Self::new(Self::DEFAULT_SIZE.get())
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Cell {
  #[default]
  Empty,
  Village {
    village: Village,
  },
}

impl Cell {
  fn village(&self) -> Option<&Village> {
    if let Self::Village { village } = self {
      Some(village)
    } else {
      None
    }
  }

  fn village_mut(&mut self) -> Option<&mut Village> {
    if let Self::Village { village } = self {
      Some(village)
    } else {
      None
    }
  }
}

impl From<Village> for Cell {
  fn from(village: Village) -> Self {
    Self::Village { village }
  }
}
