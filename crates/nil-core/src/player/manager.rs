// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Player, PlayerId};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

  pub(crate) fn insert(&mut self, player: Player) {
    self.0.insert(player.id(), player);
  }
}
