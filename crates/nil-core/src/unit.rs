// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod archer;
pub mod axeman;
pub mod heavy_cavalry;
pub mod light_cavalry;
pub mod pikeman;
pub mod prelude;
pub mod swordsman;

use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};
use std::ops::{Div, Mul, MulAssign};
use strum::EnumIter;

pub trait Unit {
  fn id(&self) -> UnitId;
  fn kind(&self) -> UnitKind;
  fn stats(&self) -> UnitStats;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub enum UnitId {
  Archer,
  Axeman,
  HeavyCavalry,
  LightCavalry,
  Pikeman,
  Swordsman,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitKind {
  Infantry,
  Cavalry,
  Ranged,
}

#[derive(Clone, Copy, Debug)]
pub struct UnitStats {
  pub attack: Power,
  pub infantry_defense: Power,
  pub cavalry_defense: Power,
  pub ranged_defense: Power,
  pub ranged_debuff: RangedDebuff,
  pub speed: Speed,
  pub haul: Haul,
}

#[derive(Clone, Copy, Debug, Deref, Into, PartialEq, Eq, Deserialize, Serialize)]
#[into(u32, f64)]
pub struct Power(u32);

impl Power {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl Mul for Power {
  type Output = Power;

  fn mul(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_mul(rhs.0))
  }
}

impl Mul<u32> for Power {
  type Output = Power;

  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_mul(rhs))
  }
}

impl Mul<Power> for u32 {
  type Output = u32;

  fn mul(self, rhs: Power) -> Self::Output {
    self.saturating_mul(rhs.0)
  }
}

impl Div for Power {
  type Output = Power;

  fn div(self, rhs: Self) -> Self::Output {
    Self(self.0.saturating_div(rhs.0))
  }
}

impl Div<u32> for Power {
  type Output = Power;

  fn div(self, rhs: u32) -> Self::Output {
    Self(self.0.saturating_div(rhs))
  }
}

impl Div<Power> for u32 {
  type Output = u32;

  fn div(self, rhs: Power) -> Self::Output {
    self.saturating_div(rhs.0)
  }
}

#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct RangedDebuff(f64);

impl RangedDebuff {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.max(0.0))
  }
}

impl Mul for RangedDebuff {
  type Output = RangedDebuff;

  fn mul(self, rhs: RangedDebuff) -> Self::Output {
    Self(self.0 * rhs.0)
  }
}

impl MulAssign for RangedDebuff {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Mul<f64> for RangedDebuff {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    self.0 * rhs
  }
}

#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
pub struct Speed(f64);

impl Speed {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.max(0.0))
  }
}

#[derive(Clone, Copy, Debug, Deref, Into, Deserialize, Serialize)]
#[into(u16, u32, f64)]
pub struct Haul(u16);

impl Haul {
  #[inline]
  pub const fn new(value: u16) -> Self {
    Self(value)
  }
}

/// Um grupo de unidades iguais.
pub struct Squad {
  unit: UnitBox,
  amount: u32,
}

impl Squad {
  pub fn new(id: UnitId, amount: u32) -> Self {
    use prelude::*;
    let unit = match id {
      UnitId::Archer => Archer::new_boxed(),
      UnitId::Axeman => Axeman::new_boxed(),
      UnitId::HeavyCavalry => HeavyCavalry::new_boxed(),
      UnitId::LightCavalry => LightCavalry::new_boxed(),
      UnitId::Pikeman => Pikeman::new_boxed(),
      UnitId::Swordsman => Swordsman::new_boxed(),
    };

    Self { unit, amount }
  }

  #[inline]
  pub fn unit(&self) -> &dyn Unit {
    &*self.unit.0
  }

  #[inline]
  pub fn kind(&self) -> UnitKind {
    self.unit.0.kind()
  }

  #[inline]
  pub fn amount(&self) -> u32 {
    self.amount
  }

  pub fn attack(&self) -> SquadAttack {
    let attack = self.unit.0.stats().attack;
    let total = f64::from(attack * self.amount);
    SquadAttack::new(total)
  }

  pub fn defense(&self) -> SquadDefense {
    let general = self.unit.0.stats().infantry_defense;
    let cavalry = self.unit.0.stats().cavalry_defense;
    let ranged = self.unit.0.stats().ranged_defense;

    let general_total = f64::from(general * self.amount);
    let cavalry_total = f64::from(cavalry * self.amount);
    let ranged_total = f64::from(ranged * self.amount);

    SquadDefense {
      infantry: general_total,
      cavalry: cavalry_total,
      ranged: ranged_total,
    }
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

pub struct SquadDefense {
  pub infantry: f64,
  pub cavalry: f64,
  pub ranged: f64,
}

#[derive(Deref)]
pub struct UnitBox(Box<dyn Unit>);

impl UnitBox {
  #[inline]
  pub fn new(unit: Box<dyn Unit>) -> Self {
    Self(unit)
  }
}
