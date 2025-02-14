use super::{Cell, World};
use crate::error::{Error, Result};
use crate::event::Event;
use crate::player::{Player, PlayerId};
use crate::village::{Coord, Village};

impl World {
  pub fn spawn_player(&mut self, player: Player) -> Result<()> {
    let id = player.id();
    if !self.players.contains_key(&id) {
      self
        .players
        .insert(id.clone(), player.clone());

      self.spawn_player_village(id)?;
      self.emit(Event::PlayerJoined { player });
    }

    Ok(())
  }

  pub fn spawn_player_village(&mut self, player: PlayerId) -> Result<()> {
    let coord = self
      .cells
      .iter()
      .position(Cell::is_empty)
      .map(|index| self.coord(index))
      .ok_or(Error::WorldIsFull)??;

    let village = Village::builder(coord)
      .name(&**player)
      .owner(player)
      .build();

    *self.cell_mut(coord)? = Cell::Village(village);

    Ok(())
  }

  pub fn get_player_villages(&self, player: &PlayerId) -> Vec<Coord> {
    self
      .cells
      .iter()
      .filter_map(|cell| cell.try_unwrap_village_ref().ok())
      .filter(|vil| vil.owner().is_some_and(|id| id == *player))
      .map(Village::coord)
      .collect()
  }
}
