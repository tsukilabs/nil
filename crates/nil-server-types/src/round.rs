// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::time::Minutes;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct RoundDuration(Minutes);

impl RoundDuration {
  pub const MIN: RoundDuration = Self(Minutes::new(5));
  pub const MAX: RoundDuration = Self(Minutes::new(60 * 12));
}

impl RoundDuration {
  pub fn new(mins: u64) -> Self {
    Self::from(Minutes::new(mins))
  }
}

impl Default for RoundDuration {
  fn default() -> Self {
    Self::MIN
  }
}

impl From<Minutes> for RoundDuration {
  fn from(mins: Minutes) -> Self {
    Self(mins).clamp(Self::MIN, Self::MAX)
  }
}

impl From<RoundDuration> for Minutes {
  fn from(duration: RoundDuration) -> Self {
    duration.0
  }
}

impl From<Duration> for RoundDuration {
  fn from(duration: Duration) -> Self {
    Self::from(Minutes::from(duration))
  }
}

impl From<RoundDuration> for Duration {
  fn from(duration: RoundDuration) -> Self {
    Duration::from(duration.0)
  }
}
