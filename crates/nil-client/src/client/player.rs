use super::Client;
use crate::error::Result;
use nil_core::{Coord, Player, PlayerConfig, PlayerId};

impl Client {
  pub async fn get_player(&self, id: PlayerId) -> Result<Player> {
    self.post_json("player", id).await
  }

  pub async fn get_player_villages(&self, id: PlayerId) -> Result<Vec<Coord>> {
    self.post_json("player/villages", id).await
  }

  pub async fn spawn_player(&self, config: PlayerConfig) -> Result<PlayerId> {
    self.put_json("player/spawn", config).await
  }
}
