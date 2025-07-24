// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::unit::stats::power::Power;
use crate::military::unit::{Unit, UnitBox, UnitId, UnitKind};
use derive_more::{Deref, From, Into};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// A group of units of the same type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Squad {
  unit: UnitBox,
  size: SquadSize,
}

impl Squad {
  pub fn new(id: UnitId, size: SquadSize) -> Self {
    Self { unit: UnitBox::from(id), size }
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

  pub fn attack(&self) -> SquadAttack {
    let attack = self.unit.stats().attack();
    let total = f64::from(attack * self.size);
    SquadAttack::new(total)
  }

  pub fn defense(&self) -> SquadDefense {
    let stats = self.unit.stats();
    let infantry = f64::from(stats.infantry_defense() * self.size);
    let cavalry = f64::from(stats.cavalry_defense() * self.size);
    let ranged = f64::from(stats.ranged_defense() * self.size);
    SquadDefense { infantry, cavalry, ranged }
  }
}

impl Add for Squad {
  type Output = Squad;

  fn add(mut self, rhs: Self) -> Self::Output {
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

impl Sub for Squad {
  type Output = Squad;

  fn sub(mut self, rhs: Self) -> Self::Output {
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

impl From<UnitId> for Squad {
  fn from(id: UnitId) -> Self {
    Squad::new(id, SquadSize::new(0))
  }
}

#[derive(Clone, Copy, Debug, Deref, Into)]
pub struct SquadAttack(f64);

impl SquadAttack {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.max(0.0))
  }
}

impl From<f64> for SquadAttack {
  fn from(value: f64) -> Self {
    Self::new(value)
  }
}

#[derive(Clone, Debug)]
pub struct SquadDefense {
  pub(crate) infantry: f64,
  pub(crate) cavalry: f64,
  pub(crate) ranged: f64,
}

#[derive(Clone, Copy, Debug, Default, Deref, From, Into, PartialEq, Eq, Deserialize, Serialize)]
pub struct SquadSize(u32);

impl SquadSize {
  #[inline]
  pub const fn new(size: u32) -> Self {
    Self(size)
  }
}

impl Add for SquadSize {
  type Output = SquadSize;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl Add<u32> for SquadSize {
  type Output = SquadSize;

  fn add(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_add(rhs))
  }
}

impl AddAssign for SquadSize {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl AddAssign<u32> for SquadSize {
  fn add_assign(&mut self, rhs: u32) {
    *self = *self + rhs;
  }
}

impl Sub for SquadSize {
  type Output = SquadSize;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl Sub<u32> for SquadSize {
  type Output = SquadSize;

  fn sub(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_sub(rhs))
  }
}

impl SubAssign for SquadSize {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl SubAssign<u32> for SquadSize {
  fn sub_assign(&mut self, rhs: u32) {
    *self = *self - rhs;
  }
}

impl Mul<Power> for SquadSize {
  type Output = Power;

  fn mul(self, rhs: Power) -> Self::Output {
    rhs * self.0
  }
}

impl Mul<SquadSize> for Power {
  type Output = Power;

  fn mul(self, rhs: SquadSize) -> Self::Output {
    self * rhs.0
  }
}
