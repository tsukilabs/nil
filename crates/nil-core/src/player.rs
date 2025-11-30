// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::resources::Resources;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlayerManager(HashMap<PlayerId, Player>);

impl PlayerManager {
  pub fn player(&self, id: &PlayerId) -> Result<&Player> {
    self
      .0
      .get(id)
      .ok_or_else(|| Error::PlayerNotFound(id.clone()))
  }

  pub(crate) fn player_mut(&mut self, id: &PlayerId) -> Result<&mut Player> {
    self
      .0
      .get_mut(id)
      .ok_or_else(|| Error::PlayerNotFound(id.clone()))
  }

  pub fn players(&self) -> impl Iterator<Item = &Player> {
    self.0.values()
  }

  pub(crate) fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
    self.0.values_mut()
  }

  #[inline]
  pub fn has(&self, id: &PlayerId) -> bool {
    self.0.contains_key(id)
  }

  pub(crate) fn spawn(&mut self, player: Player) {
    debug_assert!(!self.0.contains_key(&player.id));
    self.0.insert(player.id(), player);
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
  id: PlayerId,
  status: PlayerStatus,
  resources: Resources,
}

impl Player {
  pub fn new(options: PlayerOptions) -> Self {
    Self {
      id: options.id,
      status: PlayerStatus::Active,
      resources: Resources::PLAYER.clone(),
    }
  }

  #[inline]
  pub fn id(&self) -> PlayerId {
    self.id.clone()
  }

  #[inline]
  pub fn status(&self) -> PlayerStatus {
    self.status
  }

  pub(crate) fn status_mut(&mut self) -> &mut PlayerStatus {
    &mut self.status
  }

  #[inline]
  pub fn resources(&self) -> &Resources {
    &self.resources
  }

  pub(crate) fn resources_mut(&mut self) -> &mut Resources {
    &mut self.resources
  }

  #[inline]
  pub fn is_active(&self) -> bool {
    matches!(self.status, PlayerStatus::Active)
  }

  #[inline]
  pub fn is_inactive(&self) -> bool {
    matches!(self.status, PlayerStatus::Inactive)
  }
}

#[derive(Debug, Display, From, Into, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[from(String, &str, Arc<str>, Box<str>, Cow<'_, str>)]
pub struct PlayerId(Arc<str>);

impl Clone for PlayerId {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

impl AsRef<str> for PlayerId {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl Deref for PlayerId {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlayerStatus {
  Active,
  Inactive,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOptions {
  pub id: PlayerId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicPlayer {
  id: PlayerId,
  status: PlayerStatus,
}

impl From<&Player> for PublicPlayer {
  fn from(player: &Player) -> Self {
    Self {
      id: player.id.clone(),
      status: player.status,
    }
  }
}
