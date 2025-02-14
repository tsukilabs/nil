mod player;
mod round;

use crate::error::{Error, Result};
use crate::event::{Emitter, Event, Listener};
use crate::player::{Player, PlayerId};
use crate::round::Round;
use crate::village::{Coord, Village};
use bon::Builder;
use derive_more::TryUnwrap;
use indexmap::IndexMap;
use serde::Deserialize;
use std::num::NonZeroU8;
use strum::EnumIs;

#[derive(Debug)]
pub struct World {
  cells: Vec<Cell>,
  size: usize,
  players: IndexMap<PlayerId, Player>,
  round: Round,
  emitter: Emitter,
}

impl World {
  pub const MIN_SIZE: NonZeroU8 = NonZeroU8::new(10).unwrap();
  pub const DEFAULT_SIZE: NonZeroU8 = NonZeroU8::new(100).unwrap();

  pub fn new(config: WorldOptions) -> Self {
    let size = config.size.get().max(Self::MIN_SIZE.get());
    let size = usize::from(size);
    let capacity = size.pow(2);

    let mut cells = Vec::with_capacity(capacity);
    cells.resize_with(capacity, Cell::default);
    cells.shrink_to_fit();

    let emitter = Emitter::default();

    Self {
      cells,
      size,
      players: IndexMap::new(),
      round: Round::new(emitter.clone()),
      emitter,
    }
  }

  pub fn index(&self, coord: Coord) -> usize {
    let x = usize::from(coord.x());
    let y = usize::from(coord.y());
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

  pub fn village(&self, coord: impl Into<Coord>) -> Result<&Village> {
    let coord = coord.into();
    if let Cell::Village(village) = self.cell(coord)? {
      Ok(village)
    } else {
      Err(Error::NotAVillage(coord))
    }
  }

  pub fn village_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Village> {
    let coord = coord.into();
    if let Cell::Village(village) = self.cell_mut(coord)? {
      Ok(village)
    } else {
      Err(Error::NotAVillage(coord))
    }
  }

  pub fn player(&self, id: PlayerId) -> Result<&Player> {
    self
      .players
      .get(&id)
      .ok_or(Error::PlayerNotFound(id))
  }

  pub fn player_mut(&mut self, id: PlayerId) -> Result<&mut Player> {
    self
      .players
      .get_mut(&id)
      .ok_or(Error::PlayerNotFound(id))
  }

  pub fn round(&self) -> &Round {
    &self.round
  }

  fn emit(&self, event: Event) {
    self.emitter.emit(event);
  }

  pub fn subscribe(&self) -> Listener {
    self.emitter.subscribe()
  }
}

impl Default for World {
  fn default() -> Self {
    Self::new(WorldOptions::default())
  }
}

#[derive(Builder, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldOptions {
  #[builder(default = World::DEFAULT_SIZE)]
  pub size: NonZeroU8,
}

impl WorldOptions {
  pub fn into_world(self) -> World {
    World::new(self)
  }
}

impl Default for WorldOptions {
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
