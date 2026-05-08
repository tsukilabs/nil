// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, From, Into, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(as = "u32"))]
pub struct Minutes(u64);

impl Minutes {
  pub const fn new(mins: u64) -> Self {
    Self(mins)
  }
}

impl const PartialEq for Minutes {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl const Eq for Minutes {}

impl const PartialOrd for Minutes {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl const Ord for Minutes {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

impl const From<Duration> for Minutes {
  fn from(duration: Duration) -> Self {
    Self(duration.as_secs() / 60)
  }
}

impl const From<Minutes> for Duration {
  fn from(value: Minutes) -> Self {
    Duration::from_mins(value.0)
  }
}
