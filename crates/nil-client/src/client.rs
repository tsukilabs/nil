mod http;
mod websocket;

use crate::error::Result;
use local_ip_address::local_ip;
use nil_core::event::Event;
use nil_core::player::{Player, PlayerId, PlayerOptions};
use nil_core::round::RoundState;
use nil_core::village::{Coord, Village};
use nil_core::world::WorldState;
use std::fmt;
use std::net::{IpAddr, SocketAddrV4};
use websocket::WebSocketClient;

const USER_AGENT: &str = concat!("nil/", env!("CARGO_PKG_VERSION"));

pub struct Client {
  server_addr: SocketAddrV4,
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

  pub fn server_addr(&self) -> SocketAddrV4 {
    let mut addr = self.server_addr;
    if addr.ip().is_loopback() {
      if let Ok(IpAddr::V4(ip)) = local_ip() {
        addr.set_ip(ip);
      }
    }

    addr
  }

  /// GET `/`
  pub async fn ready(&self) -> bool {
    self
      .get("")
      .await
      .map(|()| true)
      .unwrap_or(false)
  }

  /// GET `/player`
  pub async fn players(&self) -> Result<Vec<Player>> {
    self.get_json("player").await
  }

  /// POST `/player`
  pub async fn player(&self, id: PlayerId) -> Result<Player> {
    self.post_json("player", id).await
  }

  /// PUT `/player/spawn`
  pub async fn spawn_player(&self, options: PlayerOptions) -> Result<()> {
    self.put("player/spawn", options).await
  }

  /// POST `/player/village`
  pub async fn villages_of(&self, id: PlayerId) -> Result<Vec<Coord>> {
    self.post_json("player/village", id).await
  }

  /// GET `/round`
  pub async fn round_state(&self) -> Result<RoundState> {
    self.get_json("round").await
  }

  /// GET `/version`
  pub async fn version(&self) -> Result<String> {
    self.get_text("version").await
  }

  /// POST `/village`
  pub async fn village(&self, coord: Coord) -> Result<Village> {
    self.post_json("village", coord).await
  }

  /// GET `/world`
  pub async fn world(&self) -> Result<WorldState> {
    self.get_json("world").await
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
