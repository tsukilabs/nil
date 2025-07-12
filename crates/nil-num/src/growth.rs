// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::builder;

const MIN_LEVEL: f64 = 1.0;

#[builder]
pub fn growth(
  #[builder(into)] floor: f64,
  #[builder(into)] ceil: f64,
  #[builder(into)] max_level: f64,
) -> f64 {
  debug_assert!(floor > 0.0);
  debug_assert!(ceil > floor);
  debug_assert!(max_level > MIN_LEVEL);
  ((ceil / floor).powf(1.0 / (max_level - MIN_LEVEL))) - 1.0
}
