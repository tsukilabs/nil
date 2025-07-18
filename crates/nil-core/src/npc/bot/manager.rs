// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Bot, BotId};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BotManager {
  current_id: BotId,
  map: HashMap<BotId, Bot>,
}

impl BotManager {
  pub fn bot(&self, id: BotId) -> Result<&Bot> {
    self
      .map
      .get(&id)
      .ok_or(Error::BotNotFound(id))
  }

  pub fn bots(&self) -> impl Iterator<Item = &Bot> {
    self.map.values()
  }

  pub(crate) fn spawn(&mut self) -> BotId {
    let id = self.current_id.next();
    self.map.insert(id, Bot::new(id));
    self.current_id = id;
    self.current_id
  }
}
