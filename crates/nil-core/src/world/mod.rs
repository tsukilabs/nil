mod player;
mod village;

use crate::error::{Error, Result};
use crate::player::{Player, PlayerId};
use crate::village::Village;
use bon::Builder;
use derive_more::TryUnwrap;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;
use strum::EnumIs;

#[derive(Debug)]
pub struct World {
  cells: Vec<Cell>,
  size: usize,
  players: IndexMap<PlayerId, Player>,
}

impl World {
  pub const MIN_SIZE: u8 = 10;

  pub fn new(config: WorldConfig) -> Self {
    let size = config.size.get().max(Self::MIN_SIZE);
    let size = usize::from(size);
    let capacity = size.pow(2);

    let mut cells = Vec::with_capacity(capacity);
    cells.resize_with(capacity, Cell::default);
    cells.shrink_to_fit();

    Self {
      cells,
      size,
      players: IndexMap::new(),
    }
  }

  pub fn index(&self, coord: Coord) -> usize {
    let x = usize::from(coord.x);
    let y = usize::from(coord.y);
    let index = (y * self.size) + x;
    debug_assert!(index < self.cells.len());
    index
  }

  pub fn coord(&self, index: usize) -> Result<Coord> {
    let x = index % self.size;
    let y = index / self.size;

    Ok(Coord::new(
      u8::try_from(x).map_err(|_| Error::IndexOutOfBounds(index))?,
      u8::try_from(y).map_err(|_| Error::IndexOutOfBounds(index))?,
    ))
  }

  pub fn cell(&self, coord: impl Into<Coord>) -> Result<&Cell> {
    let coord = coord.into();
    let index = self.index(coord);
    self
      .cells
      .get(index)
      .ok_or(Error::CoordOutOfBounds(coord))
  }

  pub fn cell_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Cell> {
    let coord = coord.into();
    let index = self.index(coord);
    self
      .cells
      .get_mut(index)
      .ok_or(Error::CoordOutOfBounds(coord))
  }
}

impl Default for World {
  fn default() -> Self {
    Self::new(WorldConfig::default())
  }
}

#[derive(Builder, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldConfig {
  #[builder(default = NonZeroU8::new(100).unwrap())]
  pub size: NonZeroU8,
}

impl WorldConfig {
  pub fn into_world(self) -> World {
    World::new(self)
  }
}

impl Default for WorldConfig {
  fn default() -> Self {
    Self::builder().build()
  }
}

#[derive(Debug, Default, EnumIs, TryUnwrap)]
#[try_unwrap(ref)]
pub enum Cell {
  #[default]
  #[try_unwrap(ignore)]
  Empty,
  Village(Village),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Coord {
  x: u8,
  y: u8,
}

impl Coord {
  pub fn new(x: u8, y: u8) -> Self {
    Self { x, y }
  }

  pub fn x(&self) -> u8 {
    self.x
  }

  pub fn y(&self) -> u8 {
    self.y
  }
}

impl From<(u8, u8)> for Coord {
  fn from((x, y): (u8, u8)) -> Self {
    Self::new(x, y)
  }
}
