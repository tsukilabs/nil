// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::building::{Building, BuildingId, BuildingLevel};
use crate::error::{Error, Result};
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, EnumIter};

/// Um edifício que gera recursos.
pub trait Mine: Building {
  fn mine_id(&self) -> MineId;
  /// Quantidade recursos gerados pela mina em seu nível máximo.
  fn production(&self) -> MineProduction;
  /// Taxa de crescimento da produção da mina.
  fn production_growth(&self) -> MineProductionGrowth;
  /// Quantidade recursos gerados pela mina em seu nível atual.
  fn current_production(&self, stats: &MineStatsTable) -> Result<MineProduction>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Display, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub enum MineId {
  Farm,
  IronMine,
  Quarry,
  Sawmill,
}

impl From<MineId> for BuildingId {
  fn from(value: MineId) -> Self {
    match value {
      MineId::Farm => BuildingId::Farm,
      MineId::IronMine => BuildingId::IronMine,
      MineId::Quarry => BuildingId::Quarry,
      MineId::Sawmill => BuildingId::Sawmill,
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MineStats {
  pub level: BuildingLevel,
  pub production: MineProduction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MineStatsTable {
  id: MineId,
  table: HashMap<BuildingLevel, MineStats>,
}

impl MineStatsTable {
  pub(crate) fn new(mine: &dyn Mine) -> Self {
    let max_level = *mine.max_level();
    let mut table = HashMap::with_capacity((max_level + 1).into());

    let mut production = f64::from(mine.production());
    let production_growth = *mine.production_growth();

    for level in (0..=max_level).rev() {
      let level = BuildingLevel::new(level);
      table.insert(
        level,
        MineStats {
          level,
          production: MineProduction::from(production.ceil()),
        },
      );

      debug_assert!(production.is_finite());
      debug_assert!(production >= 0.0);

      production -= production * production_growth;
    }

    table.shrink_to_fit();

    Self { id: mine.mine_id(), table }
  }

  #[inline]
  pub fn id(&self) -> MineId {
    self.id
  }

  #[inline]
  pub fn get(&self, level: BuildingLevel) -> Result<&MineStats> {
    self
      .table
      .get(&level)
      .ok_or(Error::MineStatsNotFoundForLevel(self.id, level))
  }
}

/// Quantidade de recursos gerados por uma mina num único turno.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct MineProduction(u32);

impl MineProduction {
  #[inline]
  pub const fn new(value: u32) -> Self {
    Self(value)
  }
}

impl From<MineProduction> for f64 {
  fn from(value: MineProduction) -> Self {
    f64::from(value.0)
  }
}

impl From<f64> for MineProduction {
  fn from(value: f64) -> Self {
    Self::new(value as u32)
  }
}

#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct MineProductionGrowth(f64);

impl MineProductionGrowth {
  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(0.0, 1.0))
  }
}
