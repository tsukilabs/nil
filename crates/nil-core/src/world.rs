use crate::error::{Error, Result};
use crate::village::Village;

#[derive(Debug)]
pub struct World {
  size: usize,
  cells: Vec<Cell>,
}

impl World {
  pub fn new(size: u8) -> Self {
    let size = usize::from(size);
    let capacity = size.pow(2);

    let mut cells = Vec::with_capacity(capacity);
    cells.resize_with(capacity, Cell::default);
    cells.shrink_to_fit();

    Self { cells, size }
  }

  pub fn index(&self, coord: Coord) -> usize {
    (coord.y * self.size) + coord.x
  }

  pub fn get(&self, coord: impl Into<Coord>) -> Result<&Cell> {
    let coord = coord.into();
    let index = self.index(coord);
    self
      .cells
      .get(index)
      .ok_or(Error::OutOfBounds(coord))
  }

  pub fn get_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Cell> {
    let coord = coord.into();
    let index = self.index(coord);
    self
      .cells
      .get_mut(index)
      .ok_or(Error::OutOfBounds(coord))
  }
}

#[derive(Debug, Default)]
pub enum Cell {
  #[default]
  Empty,
  Village(Village),
}

#[derive(Clone, Copy, Debug)]
pub struct Coord {
  x: usize,
  y: usize,
}

impl Coord {
  pub fn new(x: u8, y: u8) -> Self {
    Self { x: usize::from(x), y: usize::from(y) }
  }
}

impl From<(u8, u8)> for Coord {
  fn from((x, y): (u8, u8)) -> Self {
    Self::new(x, y)
  }
}
