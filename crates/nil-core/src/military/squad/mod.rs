// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod size;

use crate::error::{Error, Result};
use crate::military::unit::stats::haul::Haul;
use crate::military::unit::stats::power::{AttackPower, DefensePower, Power};
use crate::military::unit::stats::speed::Speed;
use crate::military::unit::{Unit, UnitBox, UnitId, UnitKind};
use crate::ranking::score::Score;
use crate::resources::maintenance::Maintenance;
use crate::world::config::WorldConfig;
use serde::{Deserialize, Serialize};
use size::SquadSize;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// A group of units of the same type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Squad {
  unit: UnitBox,
  size: SquadSize,
}

impl Squad {
  pub fn new(id: UnitId, size: impl Into<SquadSize>) -> Self {
    Self {
      unit: UnitBox::from(id),
      size: size.into(),
    }
  }

  #[inline]
  pub fn unit(&self) -> &dyn Unit {
    self.unit.as_dyn()
  }

  #[inline]
  pub fn id(&self) -> UnitId {
    self.unit.id()
  }

  #[inline]
  pub fn kind(&self) -> UnitKind {
    self.unit.kind()
  }

  #[inline]
  pub fn size(&self) -> SquadSize {
    self.size
  }

  #[inline]
  pub fn score(&self) -> Score {
    self.size * self.unit.score()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.size == 0u32
  }

  pub fn maintenance(&self) -> Maintenance {
    let chunk = self.unit.chunk();
    let c_size = chunk.size();
    let c_maintenance = chunk.maintenance();
    (self.size() * c_maintenance) / c_size
  }

  /// Average power of the squad.
  pub fn power(&self) -> Power {
    self.unit.power() * self.size
  }

  pub fn attack(&self) -> AttackPower {
    self.unit.attack() * self.size
  }

  pub fn defense(&self) -> DefensePower {
    let mut defense = self.unit.defense();
    defense.cavalry *= self.size;
    defense.infantry *= self.size;
    defense.ranged *= self.size;
    defense
  }

  #[inline]
  pub fn speed(&self, config: &WorldConfig) -> Speed {
    self.unit.speed(config)
  }

  #[inline]
  pub fn haul(&self) -> Haul {
    self.unit.haul() * self.size
  }

  pub fn checked_sub(&self, rhs: &Self) -> Result<Option<Self>> {
    if self.unit == rhs.unit {
      Ok(try { Self::new(self.id(), self.size.checked_sub(rhs.size)?) })
    } else {
      Err(Error::UnexpectedUnit(self.id(), rhs.id()))
    }
  }
}

impl From<UnitId> for Squad {
  fn from(id: UnitId) -> Self {
    Squad::new(id, SquadSize::new(0))
  }
}

impl Add for Squad {
  type Output = Squad;

  fn add(mut self, rhs: Self) -> Self::Output {
    self += rhs;
    self
  }
}

impl Add<SquadSize> for Squad {
  type Output = Squad;

  fn add(mut self, rhs: SquadSize) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign for Squad {
  fn add_assign(&mut self, rhs: Self) {
    if self.id() == rhs.id() {
      self.size += rhs.size;
    }
  }
}

impl AddAssign<SquadSize> for Squad {
  fn add_assign(&mut self, rhs: SquadSize) {
    self.size += rhs;
  }
}

impl Sub for Squad {
  type Output = Squad;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self -= rhs;
    self
  }
}

impl Sub<SquadSize> for Squad {
  type Output = Squad;

  fn sub(mut self, rhs: SquadSize) -> Self::Output {
    self -= rhs;
    self
  }
}

impl SubAssign for Squad {
  fn sub_assign(&mut self, rhs: Self) {
    if self.id() == rhs.id() {
      self.size -= rhs.size;
    }
  }
}

impl SubAssign<SquadSize> for Squad {
  fn sub_assign(&mut self, rhs: SquadSize) {
    self.size -= rhs;
  }
}

impl Mul<f64> for Squad {
  type Output = Squad;

  fn mul(mut self, rhs: f64) -> Self::Output {
    self *= rhs;
    self
  }
}

impl MulAssign<f64> for Squad {
  fn mul_assign(&mut self, rhs: f64) {
    self.size *= rhs;
  }
}
