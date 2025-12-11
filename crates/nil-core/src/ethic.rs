// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::{EnumIs, VariantArray};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(const)]
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
  pub const fn power(&self) -> EthicPowerAxis {
    self.power
  }

  #[inline]
  pub const fn truth(&self) -> EthicTruthAxis {
    self.truth
  }

  #[inline]
  pub const fn is_militarist(&self) -> bool {
    self.power.is_militarist()
  }

  #[inline]
  pub const fn is_fanatic_militarist(&self) -> bool {
    self.power.is_fanatic_militarist()
  }

  #[inline]
  pub const fn is_pacifist(&self) -> bool {
    self.power.is_pacifist()
  }

  #[inline]
  pub const fn is_fanatic_pacifist(&self) -> bool {
    self.power.is_fanatic_pacifist()
  }

  #[inline]
  pub const fn is_materialist(&self) -> bool {
    self.truth.is_materialist()
  }

  #[inline]
  pub const fn is_fanatic_materialist(&self) -> bool {
    self.truth.is_fanatic_materialist()
  }

  #[inline]
  pub const fn is_spiritualist(&self) -> bool {
    self.truth.is_spiritualist()
  }

  #[inline]
  pub const fn is_fanatic_spiritualist(&self) -> bool {
    self.truth.is_fanatic_spiritualist()
  }
}

impl Default for Ethics {
  fn default() -> Self {
    Self::random()
  }
}

#[derive(Clone, Copy, Debug, EnumIs, PartialEq, Eq, Deserialize, Serialize, VariantArray)]
#[serde(rename_all = "kebab-case")]
pub enum EthicPowerAxis {
  Militarist,
  FanaticMilitarist,
  Pacifist,
  FanaticPacifist,
}

impl EthicPowerAxis {
  pub fn random() -> Self {
    Self::VARIANTS
      .choose_weighted(&mut rand::rng(), |ethic| {
        match ethic {
          Self::Militarist | Self::Pacifist => 4u8,
          Self::FanaticMilitarist | Self::FanaticPacifist => 1u8,
        }
      })
      .copied()
      .expect("`Self::VARIANTS` should never be empty")
  }
}

#[derive(Clone, Copy, Debug, EnumIs, PartialEq, Eq, Deserialize, Serialize, VariantArray)]
#[serde(rename_all = "kebab-case")]
pub enum EthicTruthAxis {
  Materialist,
  FanaticMaterialist,
  Spiritualist,
  FanaticSpiritualist,
}

impl EthicTruthAxis {
  pub fn random() -> Self {
    Self::VARIANTS
      .choose_weighted(&mut rand::rng(), |ethic| {
        match ethic {
          Self::Materialist | Self::Spiritualist => 4u8,
          Self::FanaticMaterialist | Self::FanaticSpiritualist => 1u8,
        }
      })
      .copied()
      .expect("`Self::VARIANTS` should never be empty")
  }
}
