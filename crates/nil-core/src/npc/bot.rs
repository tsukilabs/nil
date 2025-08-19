// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::ethic::Ethics;
use crate::resources::Resources;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotManager(HashMap<BotId, Bot>);

impl BotManager {
  pub fn bot(&self, id: &BotId) -> Result<&Bot> {
    self
      .0
      .get(id)
      .ok_or_else(|| Error::BotNotFound(id.clone()))
  }

  pub(crate) fn bot_mut(&mut self, id: &BotId) -> Result<&mut Bot> {
    self
      .0
      .get_mut(id)
      .ok_or_else(|| Error::BotNotFound(id.clone()))
  }

  pub fn bots(&self) -> impl Iterator<Item = &Bot> {
    self.0.values()
  }

  pub(crate) fn spawn(&mut self, id: BotId) -> Result<()> {
    if self.0.contains_key(&id) {
      return Err(Error::BotAlreadySpawned(id));
    }

    self.0.insert(id.clone(), Bot::new(id));
    Ok(())
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
  id: BotId,
  ethics: Ethics,
  resources: Resources,
}

impl Bot {
  fn new(id: BotId) -> Self {
    Self {
      id,
      ethics: Ethics::random(),
      resources: Resources::BOT.clone(),
    }
  }

  #[inline]
  pub fn id(&self) -> BotId {
    self.id.clone()
  }

  #[inline]
  pub fn ethics(&self) -> &Ethics {
    &self.ethics
  }

  #[inline]
  pub fn resources(&self) -> &Resources {
    &self.resources
  }

  pub(crate) fn resources_mut(&mut self) -> &mut Resources {
    &mut self.resources
  }
}

#[derive(Debug, Display, PartialEq, Eq, Hash, From, Into, Deserialize, Serialize)]
#[from(String, &str, Arc<str>, Box<str>, Cow<'_, str>)]
pub struct BotId(Arc<str>);

impl Clone for BotId {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

impl AsRef<str> for BotId {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl Deref for BotId {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<BotId> for String {
  fn from(value: BotId) -> Self {
    String::from(value.0.as_ref())
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicBot {
  id: BotId,
}

impl From<&Bot> for PublicBot {
  fn from(bot: &Bot) -> Self {
    Self { id: bot.id.clone() }
  }
}
