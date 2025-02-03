use bon::Builder;
use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct Player {
  id: PlayerId,
  name: String,
}

impl Player {
  pub fn new(id: PlayerId, config: PlayerConfig) -> Self {
    Self { id, name: config.name }
  }

  pub fn id(&self) -> PlayerId {
    self.id
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}

#[derive(Clone, Copy, Debug, Deref, Display, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PlayerId(SocketAddr);

impl From<SocketAddr> for PlayerId {
  fn from(addr: SocketAddr) -> Self {
    Self(addr)
  }
}

#[derive(Builder, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerConfig {
  pub name: String,
}
