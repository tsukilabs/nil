// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::market::fee::MarketFee;
use crate::resources::Resources;
use derive_more::Display;
use nil_num::impl_mul_ceil;
use nil_num::mul_ceil::MulCeil;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// Gold is a special resource used to trade in the market.
#[derive(Copy, Debug, Display, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Gold(u32);

impl Gold {
  pub const MIN: Self = Self::new(0);
  pub const MAX: Self = Self::new(u32::MAX);

  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }

  #[inline]
  pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    self.0.checked_sub(rhs.0).map(Self::new)
  }
}

impl_mul_ceil!(Gold);

const impl From<u32> for Gold {
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}

const impl From<Gold> for u32 {
  fn from(value: Gold) -> Self {
    value.0
  }
}

const impl From<f64> for Gold {
  fn from(value: f64) -> Self {
    debug_assert!(value.is_finite());
    debug_assert!(value >= 0.0);
    Self(value.trunc() as u32)
  }
}

const impl From<Gold> for f64 {
  fn from(value: Gold) -> Self {
    f64::from(value.0)
  }
}

const impl From<Resources> for Gold {
  fn from(value: Resources) -> Self {
    // It's better to destructure here so the compiler will warn us
    // if we add new resources in the future and forget to update this.
    let Resources { food, iron, stone, wood } = value;

    Gold(0)
      .add(Gold::from(food))
      .add(Gold::from(iron))
      .add(Gold::from(stone))
      .add(Gold::from(wood))
  }
}

const impl PartialEq<u32> for Gold {
  fn eq(&self, other: &u32) -> bool {
    self.0.eq(other)
  }
}

const impl PartialOrd<u32> for Gold {
  fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

const impl Add for Gold {
  type Output = Gold;

  fn add(self, rhs: Gold) -> Self::Output {
    Self::new(self.0.saturating_add(rhs.0))
  }
}

const impl Add<Resources> for Gold {
  type Output = Gold;

  fn add(self, rhs: Resources) -> Self::Output {
    self + Gold::from(rhs)
  }
}

const impl AddAssign for Gold {
  fn add_assign(&mut self, rhs: Gold) {
    *self = *self + rhs;
  }
}

const impl AddAssign<Resources> for Gold {
  fn add_assign(&mut self, rhs: Resources) {
    *self = *self + rhs;
  }
}

const impl Sub for Gold {
  type Output = Gold;

  fn sub(self, rhs: Gold) -> Self::Output {
    Self::new(self.0.saturating_sub(rhs.0))
  }
}

const impl SubAssign for Gold {
  fn sub_assign(&mut self, rhs: Gold) {
    *self = *self - rhs;
  }
}

const impl Mul<u32> for Gold {
  type Output = Gold;

  fn mul(self, rhs: u32) -> Self::Output {
    Self::new(self.0.saturating_mul(rhs))
  }
}

const impl Mul<MarketFee> for Gold {
  type Output = Gold;

  fn mul(self, rhs: MarketFee) -> Self::Output {
    Self::from(self.mul_ceil(*rhs))
  }
}

impl Sum<Gold> for Gold {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Gold>,
  {
    iter.fold(Gold::default(), |acc, gold| acc + gold)
  }
}

impl Sum<Resources> for Gold {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Resources>,
  {
    iter.fold(Gold::default(), |acc, resources| acc + resources)
  }
}
