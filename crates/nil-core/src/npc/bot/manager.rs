// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::npc::bot::{Bot, BotId, BotName};
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

  pub(crate) fn spawn(&mut self) -> (BotId, BotName) {
    self.current_id = self.current_id.next();
    let entry = self
      .map
      .entry(self.current_id)
      .insert_entry(Bot::new(self.current_id));

    let bot = entry.get();

    (bot.id, bot.name.clone())
  }
}
