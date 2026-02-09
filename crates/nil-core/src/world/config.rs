// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::AnyResult;
use crate::world::WorldOptions;
use derive_more::{Deref, Display, Into};
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
  bot_density: BotDensity,
  bot_advanced_start_ratio: BotAdvancedStartRatio,
}

impl WorldConfig {
  pub fn new(options: &WorldOptions) -> Self {
    Self {
      id: WorldId::new(),
      name: options.name.clone(),
      locale: options.locale,
      allow_cheats: options.allow_cheats,
      bot_density: options.bot_density,
      bot_advanced_start_ratio: options.bot_advanced_start_ratio,
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

  #[inline]
  pub fn bot_density(&self) -> BotDensity {
    self.bot_density
  }

  #[inline]
  pub fn bot_advanced_start_ratio(&self) -> BotAdvancedStartRatio {
    self.bot_advanced_start_ratio
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

#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct BotDensity(f64);

impl BotDensity {
  pub const MIN: Self = BotDensity(0.0);
  pub const MAX: Self = BotDensity(3.0);

  #[inline]
  pub const fn new(density: f64) -> Self {
    debug_assert!(density.is_finite());
    debug_assert!(!density.is_subnormal());
    Self(density.clamp(Self::MIN.0, Self::MAX.0))
  }
}

impl Default for BotDensity {
  fn default() -> Self {
    Self::new(2.0)
  }
}

/// Proportion of bots that will have an advanced start with higher level infrastructure.
#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct BotAdvancedStartRatio(f64);

impl BotAdvancedStartRatio {
  pub const MIN: Self = BotAdvancedStartRatio(0.0);
  pub const MAX: Self = BotAdvancedStartRatio(1.0);

  #[inline]
  pub const fn new(ratio: f64) -> Self {
    debug_assert!(ratio.is_finite());
    debug_assert!(!ratio.is_subnormal());
    Self(ratio.clamp(Self::MIN.0, Self::MAX.0))
  }
}

impl Default for BotAdvancedStartRatio {
  fn default() -> Self {
    Self::new(0.2)
  }
}
