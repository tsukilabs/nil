// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod farm;
pub mod iron_mine;
pub mod prefecture;
pub mod prelude;
pub mod quarry;
pub mod sawmill;
pub mod silo;
pub mod stable;
pub mod wall;
pub mod warehouse;

use crate::error::{Error, Result};
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::resource::{
  Cost,
  Food,
  Iron,
  Maintenance,
  MaintenanceRatio,
  ResourceRatio,
  Resources,
  Stone,
  Wood,
  Workforce,
};
use derive_more::Deref;
use nil_num::growth::growth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{cmp, fmt};
use strum::{Display, EnumIter};

pub trait Building {
  fn id(&self) -> BuildingId;

  /// Verifica se o edifício está ativo.
  fn is_enabled(&self) -> bool;
  /// Ativa ou desativa o edifício.
  fn toggle(&mut self, enabled: bool);

  /// Nível atual do edifício.
  fn level(&self) -> BuildingLevel;
  /// Nível **mínimo** do edifício.
  fn min_level(&self) -> BuildingLevel;
  /// Nível **máximo** do edifício.
  fn max_level(&self) -> BuildingLevel;
  /// Define o nível do edifício, certificando-se de permanecer dentro do limite.
  fn set_level(&mut self, level: BuildingLevel);
  /// Aumenta o nível do edifício em um, se possível.
  fn increase_level(&mut self);
  /// Aumenta o nível do edifício em uma determinada quantia, se possível.
  fn increase_level_by(&mut self, amount: u8);
  /// Reduz o nível do edifício em um, se possível.
  fn decrease_level(&mut self);
  /// Reduz o nível do edifício em uma determinada quantia, se possível.
  fn decrease_level_by(&mut self, amount: u8);

  /// Custo total para o nível **mínimo** do edifício.
  fn min_cost(&self) -> Cost;
  /// Custo total para o nível **máximo** do edifício.
  fn max_cost(&self) -> Cost;
  /// Porcentagem do custo total referente à madeira.
  fn wood_ratio(&self) -> ResourceRatio;
  /// Porcentagem do custo total referente à pedra.
  fn stone_ratio(&self) -> ResourceRatio;
  /// Porcentagem do custo total referente ao ferro.
  fn iron_ratio(&self) -> ResourceRatio;

  /// Taxa de manutenção do edifício em seu nível atual.
  fn maintenance(&self, stats: &BuildingStatsTable) -> Result<Maintenance>;
  /// Proporção do custo base que deve ser usado como taxa de manutenção.
  fn maintenance_ratio(&self) -> MaintenanceRatio;

  /// Força de trabalho exigida para o nível **mínimo** do edifício.
  fn min_workforce(&self) -> Workforce;
  /// Força de trabalho exigida para o nível **máximo** do edifício.
  fn max_workforce(&self) -> Workforce;

  /// Níveis exigidos para a construção do edifício.
  fn infrastructure_requirements(&self) -> &InfrastructureRequirements;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Display, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub enum BuildingId {
  Academy,
  Farm,
  IronMine,
  Prefecture,
  Quarry,
  Sawmill,
  Silo,
  Stable,
  Wall,
  Warehouse,
}

/// Informações sobre o edifício num determinado nível.
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
      maintenance = cost * maintenance_ratio;
      workforce += workforce * workforce_growth;
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
  Clone, Copy, Debug, Default, Deref, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
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

impl From<BuildingLevel> for f64 {
  fn from(level: BuildingLevel) -> Self {
    f64::from(level.0)
  }
}

impl fmt::Display for BuildingLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
