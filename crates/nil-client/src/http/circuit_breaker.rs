// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use jiff::Timestamp;
use std::num::NonZeroU32;

/// See: <https://learn.microsoft.com/en-us/azure/architecture/patterns/circuit-breaker>
#[derive(Builder)]
pub struct CircuitBreaker {
  #[builder(skip)]
  state: CircuitState,

  #[builder(skip)]
  failure_count: u32,

  #[builder(default = unsafe { NonZeroU32::new_unchecked(25)})]
  failure_threshold: NonZeroU32,

  #[builder(skip = Timestamp::UNIX_EPOCH)]
  last_failure: Timestamp,

  #[builder(default = unsafe { NonZeroU32::new_unchecked(15_000)})]
  failure_timeout_ms: NonZeroU32,

  #[builder(skip)]
  success_count: u32,

  #[builder(default = unsafe { NonZeroU32::new_unchecked(3)})]
  success_threshold: NonZeroU32,
}

impl CircuitBreaker {
  pub fn new() -> Self {
    Self::default()
  }

  pub(super) fn update(&mut self) -> CircuitState {
    match self.state {
      CircuitState::Closed => {
        if self.failure_count >= self.failure_threshold.get() {
          self.enter(CircuitState::Open);
        }
      }
      CircuitState::Open => {
        if !self.has_recent_failure() {
          self.enter(CircuitState::HalfOpen);
        }
      }
      CircuitState::HalfOpen => {
        if self.has_recent_failure() {
          self.enter(CircuitState::Open);
        } else if self.success_count >= self.success_threshold.get() {
          self.enter(CircuitState::Closed);
        }
      }
    }

    self.state
  }

  fn enter(&mut self, state: CircuitState) {
    debug_assert_ne!(self.state, state);

    match state {
      CircuitState::Closed => {
        self.failure_count = 0;
      }
      CircuitState::Open | CircuitState::HalfOpen => {
        self.success_count = 0;
      }
    }

    self.state = state;
  }

  pub fn has_recent_failure(&self) -> bool {
    let last = self.last_failure.as_millisecond();
    let timeout = i64::from(self.failure_timeout_ms.get());

    Timestamp::now()
      .as_millisecond()
      .saturating_sub(last)
      .le(&timeout)
  }

  pub(super) fn record_failure(&mut self) {
    self.failure_count += 1;
    self.last_failure = Timestamp::now();
  }

  pub(super) fn record_success(&mut self) {
    if let CircuitState::HalfOpen = self.state {
      self.success_count += 1;
    }
  }
}

impl Default for CircuitBreaker {
  fn default() -> Self {
    Self::builder().build()
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CircuitState {
  #[default]
  Closed,
  Open,
  HalfOpen,
}
