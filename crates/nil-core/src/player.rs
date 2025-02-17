use crate::error::{Error, Result};
use derive_more::Display;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct PlayerManager(IndexMap<PlayerId, Player>);

impl PlayerManager {
  pub(crate) fn player(&self, id: PlayerId) -> Result<&Player> {
    self
      .0
      .get(&id)
      .ok_or(Error::PlayerNotFound(id))
  }

  pub(crate) fn player_mut(&mut self, id: PlayerId) -> Result<&mut Player> {
    self
      .0
      .get_mut(&id)
      .ok_or(Error::PlayerNotFound(id))
  }

  pub(crate) fn has(&self, id: &PlayerId) -> bool {
    self.0.contains_key(id)
  }

  pub(crate) fn insert(&mut self, player: Player) {
    self.0.insert(player.id(), player);
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Debug, Display, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PlayerId(Arc<str>);

impl Clone for PlayerId {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

impl Deref for PlayerId {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOptions {
  pub id: PlayerId,
}
