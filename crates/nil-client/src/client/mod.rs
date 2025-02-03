mod player;
mod village;

use crate::error::Result;
use std::net::IpAddr;

pub struct Client {
  pub(crate) server_ip: IpAddr,
}

impl Client {
  pub const SERVER_PORT: u16 = 8050;

  pub fn new(server_ip: IpAddr) -> Self {
    Client { server_ip }
  }

  pub async fn ok(&self) -> Result<bool> {
    self
      .get("")
      .await
      .map(|it| it.status().is_success())
  }

  pub async fn version(&self) -> Result<String> {
    self.get_text("version").await
  }
}
