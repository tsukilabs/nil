use super::{Cell, World};
use crate::error::{Error, Result};
use crate::event::Event;
use crate::player::{Player, PlayerId};
use crate::village::{Coord, Village};

impl World {
  pub fn spawn_player(&mut self, player: Player) -> Result<()> {
    let id = player.id();
    if self.players.contains_key(&id) {
      Err(Error::PlayerAlreadyExists)
    } else {
      self.players.insert(id, player.clone());
      self.scheduler.add_player(id);
      self.spawn_player_village(id)?;

      self
        .emitter
        .emit(Event::PlayerJoined { player });

      Ok(())
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
