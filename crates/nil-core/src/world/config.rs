// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::world::WorldOptions;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldConfig {
  name: WorldName,
  locale: Locale,
  allow_cheats: bool,
}

impl WorldConfig {
  #[inline]
  pub fn name(&self) -> WorldName {
    self.name.clone()
  }

  #[inline]
  pub fn locale(&self) -> Locale {
    self.locale
  }

  #[inline]
  pub fn are_cheats_allowed(&self) -> bool {
    self.allow_cheats
  }
}

impl From<&WorldOptions> for WorldConfig {
  fn from(options: &WorldOptions) -> Self {
    Self {
      name: options.name.clone(),
      locale: options.locale,
      allow_cheats: options.allow_cheats,
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorldName(Arc<str>);

impl<T: AsRef<str>> From<T> for WorldName {
  fn from(value: T) -> Self {
    Self(Arc::from(value.as_ref()))
  }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum Locale {
  #[default]
  #[serde(rename = "en-US")]
  English,

  #[serde(rename = "pt-BR")]
  Portuguese,
}
