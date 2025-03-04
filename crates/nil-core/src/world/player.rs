use super::World;
use crate::error::Result;
use crate::event::Event;
use crate::player::{Player, PlayerId};
use crate::village::Village;

impl World {
  pub fn spawn_player(&mut self, player: Player) -> Result<()> {
    if !self.player_manager.has(&player.id()) {
      self.player_manager.insert(player.clone());
      self.emit(Event::PlayerJoined { player });
    }

    Ok(())
  }

  fn spawn_player_village(&mut self, player: PlayerId) -> Result<()> {
    let coord = self.continent.find_empty()?;
    *self.continent.cell_mut(coord)? = Village::builder(coord)
      .name(&*player)
      .owner(player)
      .build()
      .into();

    Ok(())
  }

  pub fn remove_player(&mut self, id: &PlayerId) -> Result<()> {
    if let Some(player) = self.player_manager.remove(id) {
      self.emit(Event::PlayerLeft { player });
    }

    Ok(())
  }
}
