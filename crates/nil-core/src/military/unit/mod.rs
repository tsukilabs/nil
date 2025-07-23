// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod archer;
pub mod axeman;
pub mod heavy_cavalry;
pub mod light_cavalry;
pub mod pikeman;
pub mod prelude;
pub mod swordsman;

use crate::error::Result;
use derive_more::{Deref, Into};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Div, Mul};
use strum::EnumIter;

pub trait Unit: Send + Sync {
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

#[derive(Deref)]
pub struct UnitBox(Box<dyn Unit>);

impl UnitBox {
  #[inline]
  pub fn new(unit: Box<dyn Unit>) -> Self {
    Self(unit)
  }

  #[inline]
  pub fn as_dyn(&self) -> &dyn Unit {
    &*self.0
  }
}

impl Clone for UnitBox {
  fn clone(&self) -> Self {
    Self::from(self.0.id())
  }
}

impl fmt::Debug for UnitBox {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("UnitBox")
      .field(&self.0.id())
      .finish()
  }
}

impl From<UnitId> for UnitBox {
  fn from(id: UnitId) -> Self {
    use prelude::*;
    match id {
      UnitId::Archer => Archer::new_boxed(),
      UnitId::Axeman => Axeman::new_boxed(),
      UnitId::HeavyCavalry => HeavyCavalry::new_boxed(),
      UnitId::LightCavalry => LightCavalry::new_boxed(),
      UnitId::Pikeman => Pikeman::new_boxed(),
      UnitId::Swordsman => Swordsman::new_boxed(),
    }
  }
}

impl Serialize for UnitBox {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.id().serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for UnitBox {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(Self::from(UnitId::deserialize(deserializer)?))
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum UnitKind {
  Infantry,
  Cavalry,
  Ranged,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitStats {
  pub(crate) attack: Power,
  pub(crate) infantry_defense: Power,
  pub(crate) cavalry_defense: Power,
  pub(crate) ranged_defense: Power,
  pub(crate) ranged_debuff: RangedDebuff,
  speed: Speed,
  haul: Haul,
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

impl Mul<f64> for RangedDebuff {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    self.0 * rhs
  }
}

impl Mul<u32> for RangedDebuff {
  type Output = f64;

  fn mul(self, rhs: u32) -> Self::Output {
    self.0 * f64::from(rhs)
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
