// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::time::Minutes;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Round duration, in minutes.
#[derive(Copy, Debug, Deserialize, Serialize)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct RoundDuration(Minutes);

impl RoundDuration {
  pub const MIN: RoundDuration = Self(Minutes::new(5));
  pub const MAX: RoundDuration = Self(Minutes::new(60 * 12));
}

impl RoundDuration {
  pub const fn new(mins: u64) -> Self {
    Self::from(Minutes::new(mins))
  }
}

impl const Default for RoundDuration {
  fn default() -> Self {
    Self::MIN
  }
}

impl const From<Minutes> for RoundDuration {
  fn from(mins: Minutes) -> Self {
    Self(mins.clamp(Self::MIN.0, Self::MAX.0))
  }
}

impl const From<RoundDuration> for Minutes {
  fn from(duration: RoundDuration) -> Self {
    duration.0
  }
}

impl const From<Duration> for RoundDuration {
  fn from(duration: Duration) -> Self {
    Self::from(Minutes::from(duration))
  }
}

impl const From<RoundDuration> for Duration {
  fn from(duration: RoundDuration) -> Self {
    Duration::from(duration.0)
  }
}
