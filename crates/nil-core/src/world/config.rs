// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::AnyResult;
use crate::world::WorldOptions;
use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldConfig {
  id: WorldId,
  name: WorldName,
  locale: Locale,
  allow_cheats: bool,
}

impl WorldConfig {
  pub fn new(options: &WorldOptions) -> Self {
    Self {
      id: WorldId::new(),
      name: options.name.clone(),
      locale: options.locale,
      allow_cheats: options.allow_cheats,
    }
  }

  #[inline]
  pub fn id(&self) -> WorldId {
    self.id
  }

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

#[derive(
  Clone, Copy, Debug, Deref, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct WorldId(Uuid);

impl WorldId {
  #[must_use]
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
}

impl Default for WorldId {
  fn default() -> Self {
    Self::new()
  }
}

impl TryFrom<&str> for WorldId {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> AnyResult<Self> {
    Ok(Self(Uuid::try_parse(value)?))
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
