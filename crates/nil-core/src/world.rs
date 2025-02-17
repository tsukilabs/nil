mod player;
mod round;

use crate::continent::{Cell, Continent};
use crate::error::Result;
use crate::event::{Emitter, Event, Listener};
use crate::player::{Player, PlayerId, PlayerManager};
use crate::round::Round;
use crate::village::{Coord, Village};
use bon::Builder;
use serde::Deserialize;
use std::num::NonZeroU8;

#[derive(Debug)]
pub struct World {
  continent: Continent,
  player_manager: PlayerManager,
  round: Round,
  emitter: Emitter,
}

impl World {
  pub fn new(config: WorldOptions) -> Self {
    let continent = Continent::new(config.size.get());
    let player_manager = PlayerManager::default();
    let emitter = Emitter::default();
    let round = Round::new(emitter.clone());

    Self {
      continent,
      player_manager,
      round,
      emitter,
    }
  }

  pub fn cell(&self, coord: impl Into<Coord>) -> Result<&Cell> {
    self.continent.cell(coord)
  }

  pub fn cell_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Cell> {
    self.continent.cell_mut(coord)
  }

  pub fn village(&self, coord: impl Into<Coord>) -> Result<&Village> {
    self.continent.village(coord)
  }

  pub fn village_mut(&mut self, coord: impl Into<Coord>) -> Result<&mut Village> {
    self.continent.village_mut(coord)
  }

  pub fn player(&self, id: PlayerId) -> Result<&Player> {
    self.player_manager.player(id)
  }

  pub fn player_mut(&mut self, id: PlayerId) -> Result<&mut Player> {
    self.player_manager.player_mut(id)
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
  #[builder(default = Continent::DEFAULT_SIZE)]
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
