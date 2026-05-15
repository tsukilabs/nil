// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::military::army::Army;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::squad::Squad;
use crate::military::squad::size::SquadSize;
use crate::military::unit::stats::ranged_debuff::RangedDebuff;
use bon::Builder;
use nil_util::{ConstDeref, F64Math};
use serde::{Deserialize, Serialize};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

#[derive(Copy, Debug, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Power(u32);

impl Power {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl const From<u32> for Power {
  fn from(value: u32) -> Self {
    Power::new(value)
  }
}

impl const From<Power> for u32 {
  fn from(value: Power) -> Self {
    value.0
  }
}

impl const From<Power> for f64 {
  fn from(value: Power) -> Self {
    f64::from(value.0)
  }
}

impl<'a> Sum<&'a Squad> for Power {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Squad>,
  {
    iter.fold(Power::default(), |mut acc, squad| {
      acc += squad.power();
      acc
    })
  }
}

impl<'a> Sum<&'a ArmyPersonnel> for Power {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a ArmyPersonnel>,
  {
    iter.flat_map(ArmyPersonnel::iter).sum()
  }
}

impl<'a> Sum<&'a Army> for Power {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Army>,
  {
    iter.flat_map(Army::iter).sum()
  }
}

impl const Add for Power {
  type Output = Power;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl const AddAssign for Power {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl const Sub for Power {
  type Output = Power;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl const Mul for Power {
  type Output = Power;

  fn mul(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

impl const Mul<u32> for Power {
  type Output = Power;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_mul(rhs))
  }
}

impl const Mul<Power> for u32 {
  type Output = u32;

  fn mul(self, rhs: Power) -> Self::Output {
    self.saturating_mul(rhs.0)
  }
}

impl const Mul<RangedDebuff> for Power {
  type Output = f64;

  fn mul(self, rhs: RangedDebuff) -> Self::Output {
    f64::from(self.0) * rhs
  }
}

impl const Mul<SquadSize> for Power {
  type Output = Power;

  fn mul(self, rhs: SquadSize) -> Self::Output {
    self * u32::from(rhs)
  }
}

impl const MulAssign<u32> for Power {
  fn mul_assign(&mut self, rhs: u32) {
    *self = *self * rhs;
  }
}

impl const MulAssign<SquadSize> for Power {
  fn mul_assign(&mut self, rhs: SquadSize) {
    *self = *self * rhs;
  }
}

impl const Div for Power {
  type Output = Power;

  fn div(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_div(rhs.0))
  }
}

impl const Div<u32> for Power {
  type Output = Power;

  fn div(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_div(rhs))
  }
}

impl const Div<Power> for u32 {
  type Output = u32;

  fn div(self, rhs: Power) -> Self::Output {
    self.saturating_div(rhs.0)
  }
}

#[derive(Copy, Debug, Deserialize, Serialize, ConstDeref, F64Math)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct AttackPower(Power);

impl AttackPower {
  pub const fn new<P>(value: P) -> Self
  where
    P: [const] Into<Power>,
  {
    Self(value.into())
  }
}

impl const From<u32> for AttackPower {
  fn from(value: u32) -> Self {
    Self(Power::new(value))
  }
}

impl const From<AttackPower> for f64 {
  fn from(value: AttackPower) -> Self {
    f64::from(value.0)
  }
}

impl<'a> Sum<&'a Squad> for AttackPower {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Squad>,
  {
    iter.fold(AttackPower::default(), |mut acc, squad| {
      acc += squad.attack();
      acc
    })
  }
}

impl<'a> Sum<&'a ArmyPersonnel> for AttackPower {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a ArmyPersonnel>,
  {
    iter.flat_map(ArmyPersonnel::iter).sum()
  }
}

impl<'a> Sum<&'a Army> for AttackPower {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Army>,
  {
    iter.flat_map(Army::iter).sum()
  }
}

impl Add for AttackPower {
  type Output = AttackPower;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}

impl AddAssign for AttackPower {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl const Mul<u32> for AttackPower {
  type Output = AttackPower;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0 * rhs)
  }
}

impl const Mul<RangedDebuff> for AttackPower {
  type Output = f64;

  fn mul(self, rhs: RangedDebuff) -> Self::Output {
    self.0 * rhs
  }
}

impl const Mul<SquadSize> for AttackPower {
  type Output = AttackPower;

  fn mul(self, rhs: SquadSize) -> Self::Output {
    Self(self.0 * rhs)
  }
}

#[derive(Copy, Builder, Debug, Deserialize, Serialize)]
#[derive_const(Clone, Default, PartialEq, Eq)]
#[builder(const)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct DefensePower {
  pub(crate) cavalry: Power,
  pub(crate) infantry: Power,
  pub(crate) ranged: Power,
}

impl DefensePower {
  #[inline]
  pub const fn cavalry(&self) -> Power {
    self.cavalry
  }

  #[inline]
  pub const fn infantry(&self) -> Power {
    self.infantry
  }

  #[inline]
  pub const fn ranged(&self) -> Power {
    self.ranged
  }

  pub const fn mean(&self) -> Power {
    let defense = 0u32
      .saturating_add(*self.cavalry)
      .saturating_add(*self.infantry)
      .saturating_add(*self.ranged);

    Power::new(defense / 3)
  }
}

impl<'a> Sum<&'a Squad> for DefensePower {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Squad>,
  {
    iter.fold(DefensePower::default(), |mut acc, squad| {
      acc += squad.defense();
      acc
    })
  }
}

impl<'a> Sum<&'a ArmyPersonnel> for DefensePower {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a ArmyPersonnel>,
  {
    iter.flat_map(ArmyPersonnel::iter).sum()
  }
}

impl<'a> Sum<&'a Army> for DefensePower {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = &'a Army>,
  {
    iter.flat_map(Army::iter).sum()
  }
}

impl const Add for DefensePower {
  type Output = DefensePower;

  fn add(mut self, rhs: Self) -> Self::Output {
    self.cavalry += rhs.cavalry;
    self.infantry += rhs.infantry;
    self.ranged += rhs.ranged;
    self
  }
}

impl const AddAssign for DefensePower {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl const Mul<u32> for DefensePower {
  type Output = DefensePower;

  fn mul(mut self, rhs: u32) -> Self::Output {
    self.cavalry *= rhs;
    self.infantry *= rhs;
    self.ranged *= rhs;
    self
  }
}

impl const Mul<SquadSize> for DefensePower {
  type Output = DefensePower;

  fn mul(self, rhs: SquadSize) -> Self::Output {
    self * u32::from(rhs)
  }
}

impl const MulAssign<u32> for DefensePower {
  fn mul_assign(&mut self, rhs: u32) {
    *self = *self * rhs;
  }
}

impl const MulAssign<SquadSize> for DefensePower {
  fn mul_assign(&mut self, rhs: SquadSize) {
    *self = *self * rhs;
  }
}
