// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Error;
use bon::Builder;
use http::StatusCode;
use num_traits::ToPrimitive;
use std::num::{NonZeroU8, NonZeroU16};
use std::time::Duration;

/// See: <https://learn.microsoft.com/en-us/azure/architecture/patterns/retry>
#[derive(Builder, Clone, Debug)]
pub struct Retry {
  #[builder(default = NonZeroU8::MIN)]
  attempts: NonZeroU8,

  #[builder(default = unsafe { NonZeroU16::new_unchecked(100)})]
  min_delay: NonZeroU16,

  #[builder(default = unsafe { NonZeroU16::new_unchecked(10_000)})]
  max_delay: NonZeroU16,

  #[builder(default = 2.0)]
  multiplier: f64,

  #[builder(default = true)]
  backoff: bool,
}

impl Retry {
  pub fn with_attempts(attempts: u8) -> Self {
    let attempts = attempts.max(1);
    Self::builder()
      .attempts(unsafe { NonZeroU8::new_unchecked(attempts) })
      .build()
  }

  pub(crate) fn delay(&self, attempt: u8) -> Duration {
    debug_assert!(attempt > 0);
    debug_assert!(self.min_delay <= self.max_delay);
    debug_assert!(self.multiplier.is_normal() && self.multiplier >= 1.0);

    if self.backoff && attempt > 1 {
      let attempt = i32::from(attempt);
      let multiplier = self.multiplier.powi(attempt - 1);

      let delay = (f64::from(self.min_delay()) * multiplier)
        .min(f64::from(self.max_delay()))
        .round()
        .to_u64()
        .unwrap_or_else(|| u64::from(self.min_delay()));

      let min = u64::from(self.min_delay());
      Duration::from_millis(rand::random_range(min..=delay))
    } else {
      Duration::from_millis(u64::from(self.min_delay()))
    }
  }

  pub fn attempts(&self) -> u8 {
    self.attempts.get()
  }

  pub fn min_delay(&self) -> u16 {
    self.min_delay.get()
  }

  pub fn max_delay(&self) -> u16 {
    self.max_delay.get()
  }
}

impl Default for Retry {
  fn default() -> Self {
    Self::builder().build()
  }
}

pub(crate) fn is_retryable_status(status: StatusCode) -> bool {
  matches!(
    status,
    StatusCode::REQUEST_TIMEOUT
      | StatusCode::TOO_MANY_REQUESTS
      | StatusCode::BAD_GATEWAY
      | StatusCode::SERVICE_UNAVAILABLE
      | StatusCode::GATEWAY_TIMEOUT
  )
}

pub(crate) fn is_retryable_err(err: &Error) -> bool {
  if let Error::Reqwest(err) = err { err.is_connect() } else { false }
}
