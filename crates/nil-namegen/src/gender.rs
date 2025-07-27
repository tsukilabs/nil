// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use rand::seq::IndexedRandom;
use strum::VariantArray;

#[derive(Clone, Copy, Debug, PartialEq, Eq, VariantArray)]
pub(crate) enum Gender {
  Female,
  Male,
}

impl Gender {
  pub fn random() -> Self {
    // SAFETY: `Self::VARIANTS` will never be empty.
    unsafe {
      Self::VARIANTS
        .choose(&mut rand::rng())
        .copied()
        .unwrap_unchecked()
    }
  }
}
