use super::World;
use crate::error::Result;
use crate::event::Event;
use crate::player::{Player, PlayerId};
use crate::village::{Coord, Village};

impl World {
  pub fn spawn_player(&mut self, player: Player) -> Result<()> {
    let id = player.id();
    if !self.player_manager.has(&id) {
      self.spawn_player_village(id.clone())?;
      self.player_manager.insert(player.clone());
    }

    self.emit(Event::PlayerJoined { player });

    Ok(())
  }

  fn spawn_player_village(&mut self, player: PlayerId) -> Result<()> {
    let coord = self.continent.find_empty()?;
    *self.cell_mut(coord)? = Village::builder(coord)
      .name(&*player)
      .owner(player)
      .build()
      .into();

    Ok(())
  }

  pub fn villages_of(&self, player: &PlayerId) -> Vec<Coord> {
    self.continent.villages_of(player)
  }
}
