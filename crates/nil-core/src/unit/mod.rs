mod squad;

pub use squad::{SquadAttack, SquadDefense};

use derive_more::{Deref, Display};
use std::num::NonZeroU32;

pub trait Unit: Send + Sync {
  fn id(&self) -> UnitId;
  fn kind(&self) -> UnitKind;
  fn amount(&self) -> u32;
  fn stats(&self) -> UnitStats;

  fn squad_attack(&self) -> SquadAttack {
    let attack = self.stats().attack;
    let amount = self.amount();
    let total = f64::from(attack) * f64::from(amount);
    SquadAttack::new(total)
  }

  fn squad_defense(&self) -> SquadDefense {
    let amount = self.amount();

    let general = self.stats().general_defense;
    let cavalry = self.stats().cavalry_defense;
    let ranged = self.stats().ranged_defense;

    let general_total = f64::from(general) * f64::from(amount);
    let cavalry_total = f64::from(cavalry) * f64::from(amount);
    let ranged_total = f64::from(ranged) * f64::from(amount);

    SquadDefense {
      general: general_total,
      cavalry: cavalry_total,
      ranged: ranged_total,
    }
  }
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
  pub general_defense: Power,
  pub cavalry_defense: Power,
  pub ranged_defense: Power,
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

#[derive(Deref)]
pub struct UnitBox(Box<dyn Unit>);

impl UnitBox {
  pub fn new(unit: Box<dyn Unit>) -> Self {
    Self(unit)
  }
}
