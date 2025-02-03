use super::{Cell, Coord, World};
use crate::error::{Error, Result};
use crate::player::{Player, PlayerId};
use crate::village::Village;

impl World {
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

  pub fn spawn_player(&mut self, player: Player) -> Result<()> {
    let id = player.id();
    if self.players.contains_key(&id) {
      Err(Error::PlayerAlreadyExists)
    } else {
      self.players.insert(id, player);
      self.spawn_player_village(id)
    }
  }

  pub fn spawn_player_village(&mut self, player: PlayerId) -> Result<()> {
    let coord = self
      .cells
      .iter()
      .position(Cell::is_empty)
      .map(|index| self.coord(index))
      .ok_or(Error::WorldIsFull)??;

    let village = Village::builder(coord)
      .name(player.to_string())
      .owner(player)
      .build();

    *self.cell_mut(coord)? = Cell::Village(village);

    Ok(())
  }

  pub fn get_player_villages(&self, player: PlayerId) -> Vec<Coord> {
    self
      .cells
      .iter()
      .filter_map(|it| it.try_unwrap_village_ref().ok())
      .filter(|it| it.owner.is_some_and(|id| id == player))
      .map(|it| it.coord)
      .collect()
  }
}
