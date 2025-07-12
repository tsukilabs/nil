// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::builder;

#[builder]
pub fn growth(
  #[builder(into)] floor: f64,
  #[builder(into)] ceil: f64,
  #[builder(into)] min_level: f64,
  #[builder(into)] max_level: f64,
) -> f64 {
  ((ceil / floor).powf(1.0 / (max_level - min_level))) - 1.0
}
