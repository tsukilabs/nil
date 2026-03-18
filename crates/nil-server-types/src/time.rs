// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(
  Clone, Copy, Debug, Default, From, Into, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct Minutes(u64);

impl Minutes {
  pub const fn new(mins: u64) -> Self {
    Self(mins)
  }
}

impl From<Duration> for Minutes {
  fn from(duration: Duration) -> Self {
    Self(duration.as_secs() / 60)
  }
}

impl From<Minutes> for Duration {
  fn from(value: Minutes) -> Self {
    Duration::from_mins(value.0)
  }
}
