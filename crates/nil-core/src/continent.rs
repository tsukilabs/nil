use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::village::{Coord, Village};
use std::num::NonZeroU8;
use strum::EnumIs;

#[derive(Debug)]
pub struct Continent {
  cells: Vec<Cell>,
  size: usize,
}

impl Continent {
  pub const MIN_SIZE: NonZeroU8 = NonZeroU8::new(10).unwrap();
  pub const DEFAULT_SIZE: NonZeroU8 = NonZeroU8::new(100).unwrap();

  pub(crate) fn new(size: u8) -> Self {
    let size = size.max(Self::MIN_SIZE.get());
    let size = usize::from(size);
    let capacity = size.pow(2);

    let mut cells = Vec::with_capacity(capacity);
    cells.resize_with(capacity, Cell::default);
    cells.shrink_to_fit();

    Self { cells, size }
  }

  pub(crate) fn cell(&self, coord: impl Into<Coord>) -> Result<&Cell> {
    let coord = coord.into();
    let index = self.index(coord);
    self
      .cells
      .get(index)
      .ok_or(Error::CoordOutOfBounds(coord))
  }

  pub(crate) fn cell_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Cell> {
    let coord = coord.into();
    let index = self.index(coord);
    self
      .cells
      .get_mut(index)
      .ok_or(Error::CoordOutOfBounds(coord))
  }

  pub(crate) fn village(&self, coord: impl Into<Coord>) -> Result<&Village> {
    let coord = coord.into();
    self
      .cell(coord)?
      .village()
      .ok_or(Error::NotAVillage(coord))
  }

  pub(crate) fn village_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Village> {
    let coord = coord.into();
    self
      .cell_mut(coord)?
      .village_mut()
      .ok_or(Error::NotAVillage(coord))
  }

  pub(crate) fn villages_of(&self, player: &PlayerId) -> Vec<Coord> {
    self
      .cells
      .iter()
      .filter_map(Cell::village)
      .filter(|village| village.is_owned_by(player))
      .map(Village::coord)
      .collect()
  }

  pub(crate) fn find_empty(&self) -> Result<Coord> {
    self
      .cells
      .iter()
      .position(Cell::is_empty)
      .map(|index| self.coord(index))
      .ok_or(Error::WorldIsFull)?
  }

  fn index(&self, coord: Coord) -> usize {
    let x = usize::from(coord.x());
    let y = usize::from(coord.y());
    let index = (y * self.size) + x;
    debug_assert!(index < self.cells.len());
    index
  }

  fn coord(&self, index: usize) -> Result<Coord> {
    let x = index % self.size;
    let y = index / self.size;

    Ok(Coord::new(
      u8::try_from(x).map_err(|_| Error::IndexOutOfBounds(index))?,
      u8::try_from(y).map_err(|_| Error::IndexOutOfBounds(index))?,
    ))
  }
}

impl Default for Continent {
  fn default() -> Self {
    Self::new(Self::DEFAULT_SIZE.get())
  }
}

#[derive(Debug, Default, EnumIs)]
pub enum Cell {
  #[default]
  Empty,
  Village(Village),
}

impl Cell {
  fn village(&self) -> Option<&Village> {
    if let Self::Village(village) = self {
      Some(village)
    } else {
      None
    }
  }

  fn village_mut(&mut self) -> Option<&mut Village> {
    if let Self::Village(village) = self {
      Some(village)
    } else {
      None
    }
  }
}

impl From<Village> for Cell {
  fn from(village: Village) -> Self {
    Self::Village(village)
  }
}

#[cfg(test)]
mod tests {
  use super::Continent;
  use crate::village::Coord;

  #[test]
  fn cell() {
    each_coord(|continent, coord| {
      assert!(continent.cell(coord).is_ok());
    });
  }

  #[test]
  fn index_to_coord() {
    each_coord(|continent, coord| {
      let index = continent.index(coord);
      assert_eq!(coord, continent.coord(index).unwrap());
    });
  }

  #[test]
  fn default_continent_is_empty() {
    each_coord(|continent, coord| {
      let cell = continent.cell(coord).unwrap();
      assert!(cell.is_empty());
    });
  }

  fn each_coord(f: impl Fn(&mut Continent, Coord)) {
    let mut continent = Continent::default();
    (0..100).into_iter().for_each(|x| {
      (0..100).into_iter().for_each(|y| {
        f(&mut continent, Coord::new(x, y));
      });
    })
  }
}
