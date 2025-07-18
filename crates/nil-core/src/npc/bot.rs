// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod manager;

use crate::ethic::Ethics;
use crate::resource::Resources;
use derive_more::Display;
use serde::{Deserialize, Serialize};

pub use manager::BotManager;

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
