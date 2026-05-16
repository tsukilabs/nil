// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Round duration, in minutes.
#[derive(Copy, Debug, Deserialize, Serialize)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct RoundDuration(u16);

impl RoundDuration {
  pub const MIN: RoundDuration = Self(5);
  pub const MAX: RoundDuration = Self(60 * 12);
}

impl RoundDuration {
  pub const fn new(mins: u16) -> Self {
    Self(mins.clamp(Self::MIN.0, Self::MAX.0))
  }
}

impl const Default for RoundDuration {
  fn default() -> Self {
    Self::MIN
  }
}

impl const From<u16> for RoundDuration {
  fn from(value: u16) -> Self {
    Self::new(value)
  }
}

impl const From<Duration> for RoundDuration {
  fn from(value: Duration) -> Self {
    let mins = value.as_secs() / 60;
    u16::try_from(mins)
      .map(Self::new)
      .unwrap_or(Self::MAX)
  }
}

impl const From<RoundDuration> for Duration {
  fn from(value: RoundDuration) -> Self {
    Duration::from_mins(u64::from(value.0))
  }
}
