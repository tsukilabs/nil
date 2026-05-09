// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, From, Into, Deserialize, Serialize)]
#[derive_const(Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(as = "u32"))]
pub struct Minutes(u64);

impl Minutes {
  pub const fn new(mins: u64) -> Self {
    Self(mins)
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
