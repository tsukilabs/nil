mod player;
mod village;

use crate::error::Result;
use std::net::SocketAddrV4;

pub struct Client {
  pub(crate) server_addr: SocketAddrV4,
}

impl Client {
  pub fn new(server_addr: SocketAddrV4) -> Self {
    Client { server_addr }
  }

  pub async fn ready(&self) -> Result<bool> {
    self
      .get("")
      .await
      .map(|it| it.status().is_success())
  }

  pub async fn version(&self) -> Result<String> {
    self.get_text("version").await
  }
}
