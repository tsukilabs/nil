// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::storage::StorageCapacity;
use crate::military::squad::size::SquadSize;
use crate::resources::prelude::*;
use derive_more::{From, Into};
use nil_num::mul_ceil::MulCeil;
use nil_util::ConstDeref;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Copy, Debug, From, Into, Deserialize, Serialize, ConstDeref)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[from(u32, Food, Iron, Stone, Wood)]
#[into(u32, f64, Food, Iron, Stone, Wood)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Haul(u32);

impl Haul {
  pub const SILO_RATIO: f64 = 0.75;
  pub const WAREHOUSE_RATIO: f64 = 0.25;

  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl const PartialEq<u32> for Haul {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

impl const PartialOrd<u32> for Haul {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    Some(self.0.cmp(other))
  }
}

impl const PartialEq<StorageCapacity> for Haul {
  fn eq(&self, other: &StorageCapacity) -> bool {
    self.0.eq(&**other)
  }
}

impl const PartialOrd<StorageCapacity> for Haul {
  fn partial_cmp(&self, other: &StorageCapacity) -> Option<Ordering> {
    Some(self.0.cmp(&**other))
  }
}

impl const Add for Haul {
  type Output = Haul;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl const AddAssign for Haul {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl const Mul<SquadSize> for Haul {
  type Output = Haul;

  fn mul(self, rhs: SquadSize) -> Self::Output {
    Self(self.0.saturating_mul(*rhs))
  }
}

impl const MulAssign<SquadSize> for Haul {
  fn mul_assign(&mut self, rhs: SquadSize) {
    *self = *self * rhs;
  }
}

impl const Mul<f64> for Haul {
  type Output = Haul;

  fn mul(self, rhs: f64) -> Self::Output {
    Self(f64::from(self.0).mul_ceil(rhs) as u32)
  }
}

impl const From<StorageCapacity> for Haul {
  fn from(value: StorageCapacity) -> Self {
    Haul::new(*value)
  }
}

const _: () = {
  let mut base: f64 = 0.0;
  base += Haul::SILO_RATIO;
  base += Haul::WAREHOUSE_RATIO;
  assert!((base - 1.0).abs() < 0.001);
};
