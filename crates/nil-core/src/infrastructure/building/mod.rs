// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod farm;
pub mod iron_mine;
pub mod prefecture;
pub mod quarry;
pub mod sawmill;
pub mod silo;
pub mod stable;
pub mod wall;
pub mod warehouse;

use crate::error::{Error, Result};
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::resources::prelude::*;
use derive_more::{Deref, Into};
use nil_num::growth::growth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{cmp, fmt};
use strum::{Display, EnumIter};
use subenum::subenum;

pub trait Building: Send + Sync {
  fn id(&self) -> BuildingId;

  /// Checks whether the building is enabled.
  fn is_enabled(&self) -> bool;
  /// Enables or disables the building.
  fn toggle(&mut self, enabled: bool);

  /// Current building level.
  fn level(&self) -> BuildingLevel;
  /// Minimum building level.
  fn min_level(&self) -> BuildingLevel;
  /// Maximum building level.
  fn max_level(&self) -> BuildingLevel;
  /// Sets the building's level while ensuring it remains within the level limit.
  fn set_level(&mut self, level: BuildingLevel);
  /// Sets the building to its **minimum** level.
  fn set_min_level(&mut self);
  /// Sets the building to its **maximum** level.
  fn set_max_level(&mut self);
  /// Increases the building level by one, if possible.
  fn increase_level(&mut self);
  /// Increases the level of the building by a certain amount, if possible.
  fn increase_level_by(&mut self, amount: u8);
  /// Decreases the building level by one, if possible.
  fn decrease_level(&mut self);
  /// Decreases the level of the building by a certain amount, if possible.
  fn decrease_level_by(&mut self, amount: u8);

  /// Total cost for the **minimum** level of the building.
  fn min_cost(&self) -> Cost;
  /// Total cost for the **maximum** level of the building.
  fn max_cost(&self) -> Cost;
  /// Percentage of the total cost related to wood.
  fn wood_ratio(&self) -> ResourceRatio;
  /// Percentage of the total cost related to stone.
  fn stone_ratio(&self) -> ResourceRatio;
  /// Percentage of the total cost related to iron.
  fn iron_ratio(&self) -> ResourceRatio;

  /// Building maintenance tax at its current level.
  fn maintenance(&self, stats: &BuildingStatsTable) -> Result<Maintenance>;
  /// Proportion of the base cost used as a maintenance tax.
  fn maintenance_ratio(&self) -> MaintenanceRatio;

  /// Workforce required for the **minimum** level of the building.
  fn min_workforce(&self) -> Workforce;
  /// Workforce required for the **maximum** level of the building.
  fn max_workforce(&self) -> Workforce;

  /// Levels required to construct the building.
  fn infrastructure_requirements(&self) -> &InfrastructureRequirements;
}

#[subenum(MineId, StorageId)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Display, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum BuildingId {
  Academy,
  #[subenum(MineId)]
  Farm,
  #[subenum(MineId)]
  IronMine,
  Prefecture,
  #[subenum(MineId)]
  Quarry,
  #[subenum(MineId)]
  Sawmill,
  #[subenum(StorageId)]
  Silo,
  Stable,
  Wall,
  #[subenum(StorageId)]
  Warehouse,
}

/// Information about a building at a given level.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildingStats {
  pub level: BuildingLevel,
  pub cost: Cost,
  pub resources: Resources,
  pub maintenance: Maintenance,
  pub workforce: Workforce,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildingStatsTable {
  id: BuildingId,
  min_level: BuildingLevel,
  max_level: BuildingLevel,
  table: HashMap<BuildingLevel, BuildingStats>,
}

impl BuildingStatsTable {
  pub(crate) fn new(building: &dyn Building) -> Self {
    let min_level = building.min_level();
    let max_level = building.max_level();
    let mut table = HashMap::with_capacity((max_level.0).into());

    let mut cost = f64::from(building.min_cost());
    let cost_growth = growth()
      .floor(cost)
      .ceil(building.max_cost())
      .max_level(max_level)
      .call();

    let mut workforce = f64::from(building.min_workforce());
    let workforce_growth = growth()
      .floor(workforce)
      .ceil(building.max_workforce())
      .max_level(max_level)
      .call();

    let wood_ratio = *building.wood_ratio();
    let stone_ratio = *building.stone_ratio();
    let iron_ratio = *building.iron_ratio();

    let maintenance_ratio = *building.maintenance_ratio();
    let mut maintenance = cost * maintenance_ratio;

    for level in 1..=max_level.0 {
      let level = BuildingLevel::new(level);
      let resources = Resources {
        food: Food::MIN,
        iron: Iron::from((cost * wood_ratio).round()),
        stone: Stone::from((cost * stone_ratio).round()),
        wood: Wood::from((cost * iron_ratio).round()),
      };

      table.insert(
        level,
        BuildingStats {
          level,
          cost: Cost::from(cost.round()),
          resources,
          maintenance: Maintenance::from(maintenance.round()),
          workforce: Workforce::from(workforce.round()),
        },
      );

      debug_assert!(cost.is_normal());
      debug_assert!(workforce.is_normal());

      debug_assert!(maintenance.is_finite());
      debug_assert!(maintenance >= 0.0);

      cost += cost * cost_growth;
      workforce += workforce * workforce_growth;

      maintenance = cost * maintenance_ratio;
    }

    table.shrink_to_fit();

    Self {
      id: building.id(),
      min_level,
      max_level,
      table,
    }
  }

  #[inline]
  pub fn id(&self) -> BuildingId {
    self.id
  }

  #[inline]
  pub fn min_level(&self) -> BuildingLevel {
    self.min_level
  }

  #[inline]
  pub fn max_level(&self) -> BuildingLevel {
    self.max_level
  }

  #[inline]
  pub fn get(&self, level: BuildingLevel) -> Result<&BuildingStats> {
    self
      .table
      .get(&level)
      .ok_or(Error::BuildingStatsNotFoundForLevel(self.id, level))
  }
}

#[derive(
  Clone,
  Copy,
  Debug,
  Default,
  Deref,
  Into,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Deserialize,
  Serialize,
)]
#[into(u8, u16, u32, u64, usize, f64)]
pub struct BuildingLevel(u8);

impl BuildingLevel {
  pub const ZERO: BuildingLevel = BuildingLevel(0);

  #[inline]
  pub const fn new(level: u8) -> Self {
    Self(level)
  }
}

impl PartialEq<u8> for BuildingLevel {
  fn eq(&self, other: &u8) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<u8> for BuildingLevel {
  fn partial_cmp(&self, other: &u8) -> Option<cmp::Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Add for BuildingLevel {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self(self.0.saturating_add(rhs.0))
  }
}

impl AddAssign for BuildingLevel {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Sub for BuildingLevel {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self(self.0.saturating_sub(rhs.0))
  }
}

impl SubAssign for BuildingLevel {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Add<u8> for BuildingLevel {
  type Output = Self;

  fn add(self, rhs: u8) -> Self {
    Self(self.0.saturating_add(rhs))
  }
}

impl Add<i8> for BuildingLevel {
  type Output = Self;

  fn add(self, rhs: i8) -> Self {
    Self(self.0.saturating_add_signed(rhs))
  }
}

impl AddAssign<u8> for BuildingLevel {
  fn add_assign(&mut self, rhs: u8) {
    *self = *self + rhs;
  }
}

impl AddAssign<i8> for BuildingLevel {
  fn add_assign(&mut self, rhs: i8) {
    *self = *self + rhs;
  }
}

impl Sub<u8> for BuildingLevel {
  type Output = Self;

  fn sub(self, rhs: u8) -> Self {
    Self(self.0.saturating_sub(rhs))
  }
}

impl Sub<i8> for BuildingLevel {
  type Output = Self;

  fn sub(self, rhs: i8) -> Self {
    Self(self.0.saturating_sub_signed(rhs))
  }
}

impl SubAssign<u8> for BuildingLevel {
  fn sub_assign(&mut self, rhs: u8) {
    *self = *self - rhs;
  }
}

impl SubAssign<i8> for BuildingLevel {
  fn sub_assign(&mut self, rhs: i8) {
    *self = *self - rhs;
  }
}

impl fmt::Display for BuildingLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/// Shorthand for [`BuildingLevel::new`].
#[macro_export]
macro_rules! lv {
  ($level:expr) => {
    const { $crate::infrastructure::building::BuildingLevel::new($level) }
  };
}

/// Creates a building with a random level.
#[macro_export]
macro_rules! with_random_level {
  ($building:ident) => {{ $crate::infrastructure::prelude::$building::with_random_level() }};
  ($building:ident, $max:expr) => {{
    $crate::infrastructure::prelude::$building::with_random_level_in()
      .max($crate::lv!($max))
      .call()
  }};
  ($building:ident, $min:expr, $max:expr) => {{
    $crate::infrastructure::prelude::$building::with_random_level_in()
      .min($crate::lv!($min))
      .max($crate::lv!($max))
      .call()
  }};
}
