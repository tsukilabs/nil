// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Infrastructure;
use super::building::wall::WallStatsTable;
use super::building::{BuildingId, BuildingStatsTable, MineId, StorageId};
use super::mine::MineStatsTable;
use super::storage::StorageStatsTable;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfrastructureStats {
  building: HashMap<BuildingId, BuildingStatsTable>,
  mine: HashMap<MineId, MineStatsTable>,
  storage: HashMap<StorageId, StorageStatsTable>,
  wall: WallStatsTable,
}

impl InfrastructureStats {
  pub fn new() -> Self {
    let infrastructure = Infrastructure::default();
    let building = BuildingId::iter()
      .map(|id| (id, BuildingStatsTable::new(infrastructure.building(id))))
      .collect();

    let mine = MineId::iter()
      .map(|id| (id, MineStatsTable::new(infrastructure.mine(id))))
      .collect();

    let storage = StorageId::iter()
      .map(|id| (id, StorageStatsTable::new(infrastructure.storage(id))))
      .collect();

    let wall = WallStatsTable::new();

    Self { building, mine, storage, wall }
  }

  #[inline]
  pub fn building(&self, id: BuildingId) -> Result<&BuildingStatsTable> {
    self
      .building
      .get(&id)
      .ok_or(Error::BuildingStatsNotFound(id))
  }

  #[inline]
  pub fn mine(&self, id: MineId) -> Result<&MineStatsTable> {
    self
      .mine
      .get(&id)
      .ok_or(Error::MineStatsNotFound(id))
  }

  #[inline]
  pub fn storage(&self, id: StorageId) -> Result<&StorageStatsTable> {
    self
      .storage
      .get(&id)
      .ok_or(Error::StorageStatsNotFound(id))
  }
}

impl Default for InfrastructureStats {
  fn default() -> Self {
    Self::new()
  }
}
