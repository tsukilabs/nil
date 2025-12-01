// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod size;

use crate::military::unit::{Unit, UnitBox, UnitId, UnitKind};
use crate::ranking::Score;
use crate::resources::Maintenance;
use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub use size::SquadSize;

/// A group of units of the same type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
