// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod archer;
pub mod axeman;
pub mod heavy_cavalry;
pub mod light_cavalry;
pub mod pikeman;
pub mod prelude;
pub mod stats;
pub mod swordsman;

use crate::error::Result;
use crate::resources::prelude::*;
use derive_more::{Deref, From, Into};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use stats::prelude::*;
use std::fmt;
use strum::EnumIter;

pub trait Unit: Send + Sync {
  fn id(&self) -> UnitId;
  fn kind(&self) -> UnitKind;

  fn stats(&self) -> &UnitStats;
  fn attack(&self) -> Power;
  fn infantry_defense(&self) -> Power;
  fn cavalry_defense(&self) -> Power;
  fn ranged_defense(&self) -> Power;
  fn ranged_debuff(&self) -> RangedDebuff;
  fn speed(&self) -> Speed;
  fn haul(&self) -> Haul;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitChunk {
  size: UnitChunkSize,
  cost: Cost,
  wood_ratio: ResourceRatio,
  stone_ratio: ResourceRatio,
  iron_ratio: ResourceRatio,
  maintenance_ratio: MaintenanceRatio,
  workforce: Workforce,
}

#[derive(Clone, Copy, Debug, Deref, From, Into, Deserialize, Serialize)]
pub struct UnitChunkSize(u8);

impl UnitChunkSize {
  #[inline]
  pub const fn new(size: u8) -> UnitChunkSize {
    UnitChunkSize(size)
  }
}
