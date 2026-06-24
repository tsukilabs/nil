// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::army::Army;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::squad::Squad;
use derive_more::Display;
use nil_num::impl_mul_ceil;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Score(u32);

impl Score {
  pub const ZERO: Score = Score(0);

  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

const impl From<u32> for Score {
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}

const impl From<Score> for u32 {
  fn from(value: Score) -> Self {
    value.0
  }
}

const impl From<f64> for Score {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

const impl From<Score> for f64 {
  fn from(value: Score) -> Self {
    f64::from(value.0)
  }
}

const impl PartialEq<u32> for Score {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

const impl PartialOrd<u32> for Score {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl<'a> Sum<&'a Squad> for Score {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Squad>,
  {
    iter.fold(Score::default(), |mut acc, squad| {
      acc += squad.score();
      acc
    })
  }
}

impl<'a> Sum<&'a ArmyPersonnel> for Score {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a ArmyPersonnel>,
  {
    iter.flat_map(ArmyPersonnel::iter).sum()
  }
}

impl<'a> Sum<&'a Army> for Score {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Army>,
  {
    iter.flat_map(Army::iter).sum()
  }
}

const impl Add for Score {
  type Output = Score;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

const impl AddAssign for Score {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

const impl Add<u32> for Score {
  type Output = Score;

  fn add(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_add(rhs))
  }
}

const impl AddAssign<u32> for Score {
  fn add_assign(&mut self, rhs: u32) {
    *self = *self + rhs;
  }
}

const impl Sub for Score {
  type Output = Score;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

const impl SubAssign for Score {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

const impl Mul for Score {
  type Output = Score;

  fn mul(self, rhs: Score) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

const impl Mul<u32> for Score {
  type Output = Score;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_mul(rhs))
  }
}

const impl MulAssign for Score {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl_mul_ceil!(Score);
