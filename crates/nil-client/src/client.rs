mod http;
mod websocket;

use crate::error::Result;
use nil_core::event::Event;
use nil_core::player::{Player, PlayerId, PlayerOptions};
use nil_core::round::RoundState;
use nil_core::village::{Coord, Village};
use std::fmt;
use std::net::SocketAddrV4;
use websocket::WebSocketClient;

const USER_AGENT: &str = concat!("nil/", env!("CARGO_PKG_VERSION"));

pub struct Client {
  pub(crate) server_addr: SocketAddrV4,
  websocket: WebSocketClient,
}

impl Client {
  pub async fn start<F>(server_addr: SocketAddrV4, on_event: F) -> Result<Self>
  where
    F: Fn(Event) + Send + Sync + 'static,
  {
    Ok(Client {
      server_addr,
      websocket: WebSocketClient::connect(&server_addr, on_event).await?,
    })
  }

  /// GET `/`
  pub async fn ready(&self) -> bool {
    self
      .get("")
      .await
      .map(|()| true)
      .unwrap_or(false)
  }

  /// POST `/player`
  pub async fn get_player(&self, id: PlayerId) -> Result<Player> {
    self.post_json("player", id).await
  }

  /// PUT `/player/spawn`
  pub async fn spawn_player(&self, options: PlayerOptions) -> Result<()> {
    self.put("player/spawn", options).await
  }

  /// POST `/player/village`
  pub async fn get_player_villages(&self, id: PlayerId) -> Result<Vec<Coord>> {
    self.post_json("player/village", id).await
  }

  /// GET `/round`
  pub async fn get_round_state(&self) -> Result<RoundState> {
    self.get_json("round").await
  }

  /// GET `/version`
  pub async fn version(&self) -> Result<String> {
    self.get_text("version").await
  }

  /// POST `/village`
  pub async fn get_village(&self, coord: Coord) -> Result<Village> {
    self.post_json("village", coord).await
  }
}

impl fmt::Debug for Client {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Client")
      .field("server_addr", &self.server_addr)
      .field("websocket", &self.websocket)
      .finish()
  }
}
