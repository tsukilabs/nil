// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod manager;

use crate::ethic::Ethics;
use crate::resource::Resources;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

pub use manager::BotManager;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
  id: BotId,
  name: BotName,
  ethics: Ethics,
  resources: Resources,
}

impl Bot {
  fn new(id: BotId) -> Self {
    // TODO: These guys deserve better names.
    let name = format!("Bot {id}");
    Self {
      id,
      name: BotName(Arc::from(name)),
      ethics: Ethics::random(),
      resources: Resources::BOT.clone(),
    }
  }

  #[inline]
  pub fn ethics(&self) -> &Ethics {
    &self.ethics
  }

  #[inline]
  pub fn resources(&self) -> &Resources {
    &self.resources
  }
}

#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BotId(u32);

impl BotId {
  #[inline]
  #[must_use]
  const fn next(self) -> Self {
    Self(self.0.saturating_add(1))
  }
}

#[derive(Debug, Display, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotName(Arc<str>);

impl Clone for BotName {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

impl AsRef<str> for BotName {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl Deref for BotName {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<BotName> for String {
  fn from(value: BotName) -> Self {
    String::from(value.0.as_ref())
  }
}
