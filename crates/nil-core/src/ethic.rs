// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::VariantArray;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ethics {
  power: EthicPowerAxis,
  truth: EthicTruthAxis,
}

impl Ethics {
  pub fn random() -> Self {
    Self {
      power: EthicPowerAxis::random(),
      truth: EthicTruthAxis::random(),
    }
  }

  #[inline]
  pub fn power(&self) -> EthicPowerAxis {
    self.power
  }

  #[inline]
  pub fn truth(&self) -> EthicTruthAxis {
    self.truth
  }
}

impl Default for Ethics {
  fn default() -> Self {
    Self::random()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, VariantArray)]
#[serde(rename_all = "kebab-case")]
pub enum EthicPowerAxis {
  Militarist,
  Pacifist,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, VariantArray)]
#[serde(rename_all = "kebab-case")]
pub enum EthicTruthAxis {
  Materialist,
  Spiritualist,
}

macro_rules! impl_axis {
  ($($axis:ident),+ $(,)?) => {
    $(
      impl $axis {
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
    )+
  };
}

impl_axis!(EthicPowerAxis, EthicTruthAxis);
