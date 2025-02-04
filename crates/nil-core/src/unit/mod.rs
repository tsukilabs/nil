mod archer;
mod axeman;
mod heavy_cavalry;
mod light_cavalry;
mod pikeman;
mod swordsman;

use crate::error::{Error, Result};
use derive_more::{Deref, Display};
use std::num::NonZeroU32;

pub trait Unit: Send + Sync {
  fn id(&self) -> UnitId;
  fn kind(&self) -> UnitKind;
  fn stats(&self) -> UnitStats;
}

#[derive(Clone, Copy, Debug, Deref, Display)]
pub struct UnitId(NonZeroU32);

impl UnitId {
  pub const fn new(value: u32) -> Self {
    Self(NonZeroU32::new(value).unwrap())
  }
}

impl From<u32> for UnitId {
  fn from(value: u32) -> Self {
    Self::new(value)
  }
}

impl From<i32> for UnitId {
  fn from(value: i32) -> Self {
    Self::new(value.unsigned_abs())
  }
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
  pub ranged_debuff: f64,
  pub speed: Speed,
  pub haul: Haul,
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct Power(u16);

impl Power {
  pub const fn new(value: u16) -> Self {
    Self(value)
  }
}

impl From<Power> for f64 {
  fn from(power: Power) -> Self {
    f64::from(power.0)
  }
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct Speed(f64);

impl Speed {
  pub const fn new(value: f64) -> Self {
    Self(value.max(0.0))
  }
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct Haul(u16);

impl Haul {
  pub const fn new(value: u16) -> Self {
    Self(value)
  }
}

impl From<Haul> for f64 {
  fn from(haul: Haul) -> Self {
    f64::from(haul.0)
  }
}

/// A group of the same type of unit.
pub struct Squad {
  unit: UnitBox,
  amount: u32,
}

impl Squad {
  pub fn new<Id>(id: Id, amount: u32) -> Result<Self>
  where
    Id: Into<UnitId>,
  {
    let id: UnitId = id.into();
    let unit = match id.get() {
      1 => pikeman::Pikeman::new_boxed(),
      2 => swordsman::Swordsman::new_boxed(),
      3 => axeman::Axeman::new_boxed(),
      4 => archer::Archer::new_boxed(),
      5 => light_cavalry::LightCavalry::new_boxed(),
      6 => heavy_cavalry::HeavyCavalry::new_boxed(),
      _ => return Err(Error::UnitNotFound(id)),
    };

    Ok(Self { unit, amount })
  }

  /// # Safety
  ///
  /// Callers must ensure that the given `id` is valid.
  pub unsafe fn new_unchecked<Id>(id: Id, amount: u32) -> Self
  where
    Id: Into<UnitId>,
  {
    unsafe { Self::new(id, amount).unwrap_unchecked() }
  }

  pub fn unit(&self) -> &dyn Unit {
    &*self.unit.0
  }

  pub fn kind(&self) -> UnitKind {
    self.unit.0.kind()
  }

  pub fn amount(&self) -> u32 {
    self.amount
  }

  pub fn attack(&self) -> SquadAttack {
    let attack = self.unit.0.stats().attack;
    let total = f64::from(attack) * f64::from(self.amount);
    SquadAttack::new(total)
  }

  pub fn defense(&self) -> SquadDefense {
    let amount = self.amount;

    let general = self.unit.0.stats().infantry_defense;
    let cavalry = self.unit.0.stats().cavalry_defense;
    let ranged = self.unit.0.stats().ranged_defense;

    let general_total = f64::from(general) * f64::from(amount);
    let cavalry_total = f64::from(cavalry) * f64::from(amount);
    let ranged_total = f64::from(ranged) * f64::from(amount);

    SquadDefense {
      infantry: general_total,
      cavalry: cavalry_total,
      ranged: ranged_total,
    }
  }
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct SquadAttack(f64);

impl SquadAttack {
  pub fn new(value: f64) -> Self {
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
  pub fn new(unit: Box<dyn Unit>) -> Self {
    Self(unit)
  }
}
