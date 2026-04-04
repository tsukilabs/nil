// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::AnyResult;
use crate::world::WorldOptions;
use bon::Builder;
use derive_more::{Deref, Display, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldConfig {
  #[builder(start_fn, into)]
  name: WorldName,

  #[builder(skip)]
  id: WorldId,

  #[serde(default)]
  #[builder(default)]
  locale: Locale,

  #[serde(default)]
  #[builder(default)]
  allow_cheats: bool,

  #[serde(default)]
  #[builder(default)]
  speed: WorldSpeed,

  #[serde(default)]
  #[builder(default)]
  bot_density: BotDensity,

  #[serde(default)]
  #[builder(default)]
  bot_advanced_start_ratio: BotAdvancedStartRatio,
}

impl WorldConfig {
  pub fn new(options: &WorldOptions) -> Self {
    Self {
      id: WorldId::new(),
      name: options.name.clone(),
      locale: options.locale,
      allow_cheats: options.allow_cheats,
      speed: options.speed,
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
  pub fn speed(&self) -> WorldSpeed {
    self.speed
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
pub struct WorldName(Box<str>);

impl<T: AsRef<str>> From<T> for WorldName {
  fn from(value: T) -> Self {
    Self(Box::from(value.as_ref()))
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

macro_rules! impl_f64_newtype {
  ($name:ident, min = $min:expr, max = $max:expr) => {
    impl $name {
      pub const MIN: Self = $name($min);
      pub const MAX: Self = $name($max);

      #[inline]
      pub const fn new(value: f64) -> Self {
        debug_assert!(value.is_finite());
        debug_assert!(!value.is_subnormal());
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
      }
    }
  };
  ($name:ident, min = $min:expr, max = $max:expr, default = $default:expr) => {
    impl_f64_newtype!($name, min = $min, max = $max);

    impl Default for $name {
      fn default() -> Self {
        Self::new($default)
      }
    }
  };
}

#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize, nil_num::F64Ops)]
pub struct WorldSpeed(f64);

impl_f64_newtype!(WorldSpeed, min = 0.5, max = 5.0, default = 1.0);

#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize, nil_num::F64Ops)]
pub struct BotDensity(f64);

impl_f64_newtype!(
  BotDensity,
  min = 0.0,
  max = 3.0,
  default = if cfg!(target_os = "android") { 1.0 } else { 2.0 }
);

/// Proportion of bots that will have an advanced start with higher level infrastructure.
#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize, nil_num::F64Ops)]
pub struct BotAdvancedStartRatio(f64);

impl_f64_newtype!(BotAdvancedStartRatio, min = 0.0, max = 1.0, default = 0.2);
