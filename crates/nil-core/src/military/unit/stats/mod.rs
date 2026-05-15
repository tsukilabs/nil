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

use crate::military::unit::stats::power::DefensePower;

#[derive(Builder, Debug, Deserialize, Serialize)]
#[derive_const(Clone)]
#[serde(rename_all = "camelCase")]
#[builder(const)]
pub struct UnitStats {
  attack: AttackPower,
  defense: DefensePower,
  ranged_debuff: RangedDebuff,
  base_speed: Speed,
  haul: Haul,
}

impl UnitStats {
  /// Average power.
  #[inline]
  pub const fn power(&self) -> Power {
    (*self.attack + self.defense().mean()) / 2
  }

  #[inline]
  pub const fn attack(&self) -> AttackPower {
    self.attack
  }

  #[inline]
  pub const fn defense(&self) -> DefensePower {
    self.defense
  }

  #[inline]
  pub const fn ranged_debuff(&self) -> RangedDebuff {
    self.ranged_debuff
  }

  #[inline]
  pub const fn base_speed(&self) -> Speed {
    self.base_speed
  }

  #[inline]
  pub const fn haul(&self) -> Haul {
    self.haul
  }
}
