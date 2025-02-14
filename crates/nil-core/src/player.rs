use bon::Builder;
use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct Player {
  id: PlayerId,
}

impl Player {
  pub fn id(&self) -> PlayerId {
    self.id.clone()
  }
}

impl From<PlayerOptions> for Player {
  fn from(options: PlayerOptions) -> Self {
    Player { id: options.id }
  }
}

#[derive(Debug, Deref, Display, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PlayerId(Arc<str>);

impl Clone for PlayerId {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

#[derive(Builder, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOptions {
  pub id: PlayerId,
}
