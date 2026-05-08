// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod archer;
pub mod axeman;
pub mod heavy_cavalry;
pub mod light_cavalry;
pub mod pikeman;
pub mod prelude;
pub mod ram;
pub mod stats;
pub mod swordsman;

use crate::error::Result;
use crate::infrastructure::prelude::BuildingId;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::ranking::score::Score;
use crate::resources::prelude::*;
use crate::world::config::WorldConfig;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use stats::prelude::*;
use std::fmt;
use std::num::NonZeroU32;
use std::ops::{Deref, Mul};
use strum::EnumIter;
use subenum::subenum;

pub trait Unit: Send + Sync {
  fn id(&self) -> UnitId;
  fn kind(&self) -> UnitKind;

  /// Building where the unit is recruited.
  fn building(&self) -> BuildingId;

  fn score(&self) -> Score;

  fn stats(&self) -> &UnitStats;
  fn attack(&self) -> Power;
  fn infantry_defense(&self) -> Power;
  fn cavalry_defense(&self) -> Power;
  fn ranged_defense(&self) -> Power;
  fn ranged_debuff(&self) -> RangedDebuff;
  fn speed(&self, config: &WorldConfig) -> Speed;
  fn haul(&self) -> Haul;

  fn chunk(&self) -> &UnitChunk;

  /// Building levels required to recruit the unit.
  fn infrastructure_requirements(&self) -> &InfrastructureRequirements;

  fn is_cavalry(&self) -> bool {
    matches!(self.kind(), UnitKind::Cavalry)
  }

  fn is_infantry(&self) -> bool {
    matches!(self.kind(), UnitKind::Infantry)
  }

  fn is_ranged(&self) -> bool {
    matches!(self.kind(), UnitKind::Ranged)
  }

  fn is_offensive(&self) -> bool {
    match self.id() {
      UnitId::Archer | UnitId::Ram => true,
      _ => {
        let infantry = self.infantry_defense();
        let cavalry = self.cavalry_defense();
        self.attack() > infantry.max(cavalry)
      }
    }
  }

  fn is_defensive(&self) -> bool {
    match self.id() {
      UnitId::Archer => true,
      UnitId::Ram => false,
      _ => !self.is_offensive(),
    }
  }

  fn is_academy_unit(&self) -> bool {
    matches!(self.building(), BuildingId::Academy)
  }

  fn is_stable_unit(&self) -> bool {
    matches!(self.building(), BuildingId::Stable)
  }

  fn is_workshop_unit(&self) -> bool {
    matches!(self.building(), BuildingId::Workshop)
  }
}

#[subenum(AcademyUnitId, StableUnitId, WorkshopUnitId)]
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
#[remain::sorted]
pub enum UnitId {
  #[subenum(AcademyUnitId)]
  Archer,

  #[subenum(AcademyUnitId)]
  Axeman,

  #[subenum(StableUnitId)]
  HeavyCavalry,

  #[subenum(StableUnitId)]
  LightCavalry,

  #[subenum(AcademyUnitId)]
  Pikeman,

  #[subenum(WorkshopUnitId)]
  Ram,

  #[subenum(AcademyUnitId)]
  Swordsman,
}

impl From<UnitId> for BuildingId {
  fn from(id: UnitId) -> Self {
    UnitBox::from(id).0.building()
  }
}

#[derive(derive_more::Deref)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(as = "UnitId"))]
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

impl PartialEq for UnitBox {
  fn eq(&self, other: &Self) -> bool {
    self.0.id() == other.0.id()
  }
}

impl Eq for UnitBox {}

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
      UnitId::Ram => Ram::new_boxed(),
      UnitId::Swordsman => Swordsman::new_boxed(),
    }
  }
}

impl From<AcademyUnitId> for UnitBox {
  fn from(id: AcademyUnitId) -> Self {
    Self::from(UnitId::from(id))
  }
}

impl From<StableUnitId> for UnitBox {
  fn from(id: StableUnitId) -> Self {
    Self::from(UnitId::from(id))
  }
}

impl From<WorkshopUnitId> for UnitBox {
  fn from(id: WorkshopUnitId) -> Self {
    Self::from(UnitId::from(id))
  }
}

impl Serialize for UnitBox {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.0.id().serialize(serializer)
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
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum UnitKind {
  Infantry,
  Cavalry,
  Ranged,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct UnitChunk {
  size: UnitChunkSize,
  cost: Cost,
  food_ratio: ResourceRatio,
  iron_ratio: ResourceRatio,
  stone_ratio: ResourceRatio,
  wood_ratio: ResourceRatio,
  maintenance_ratio: MaintenanceRatio,
  workforce: Workforce,
}

impl UnitChunk {
  #[inline]
  pub fn size(&self) -> UnitChunkSize {
    self.size
  }

  pub fn resources(&self) -> Resources {
    Resources {
      food: Food::from((self.cost * self.food_ratio).round()),
      iron: Iron::from((self.cost * self.iron_ratio).round()),
      stone: Stone::from((self.cost * self.stone_ratio).round()),
      wood: Wood::from((self.cost * self.wood_ratio).round()),
    }
  }

  #[inline]
  pub fn maintenance(&self) -> Maintenance {
    Maintenance::from((self.cost * self.maintenance_ratio).round())
  }

  #[inline]
  pub fn workforce(&self) -> Workforce {
    self.workforce
  }
}

#[derive(Clone, Copy, Debug, From, Into, Deserialize, Serialize)]
#[into(u8, u32)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct UnitChunkSize(u8);

impl UnitChunkSize {
  #[inline]
  pub const fn new(size: u8) -> UnitChunkSize {
    UnitChunkSize(size)
  }
}

impl const Deref for UnitChunkSize {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl const From<UnitChunkSize> for f64 {
  fn from(value: UnitChunkSize) -> Self {
    f64::from(value.0)
  }
}

impl const Mul<u32> for UnitChunkSize {
  type Output = u32;

  fn mul(self, rhs: u32) -> Self::Output {
    u32::from(self.0).saturating_mul(rhs)
  }
}

impl const Mul<NonZeroU32> for UnitChunkSize {
  type Output = u32;

  fn mul(self, rhs: NonZeroU32) -> Self::Output {
    self * rhs.get()
  }
}
