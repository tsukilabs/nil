// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod haul;
pub mod power;
pub mod prelude;
pub mod ranged_debuff;
pub mod speed;

use bon::Builder;
use prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(const)]
pub struct UnitStats {
  attack: Power,
  infantry_defense: Power,
  cavalry_defense: Power,
  ranged_defense: Power,
  ranged_debuff: RangedDebuff,
  speed: Speed,
  haul: Haul,
}

impl UnitStats {
  #[inline]
  pub fn attack(&self) -> Power {
    self.attack
  }

  #[inline]
  pub fn infantry_defense(&self) -> Power {
    self.infantry_defense
  }

  #[inline]
  pub fn cavalry_defense(&self) -> Power {
    self.cavalry_defense
  }

  #[inline]
  pub fn ranged_defense(&self) -> Power {
    self.ranged_defense
  }

  #[inline]
  pub fn ranged_debuff(&self) -> RangedDebuff {
    self.ranged_debuff
  }

  #[inline]
  pub fn speed(&self) -> Speed {
    self.speed
  }

  #[inline]
  pub fn haul(&self) -> Haul {
    self.haul
  }
}
