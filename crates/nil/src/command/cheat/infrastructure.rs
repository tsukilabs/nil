// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::building::{BuildingId, BuildingLevel};
use nil_core::village::Coord;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_set_building_level(
  app: AppHandle,
  coord: Coord,
  id: BuildingId,
  level: BuildingLevel,
) -> Result<()> {
  app
    .client(async |cl| {
      cl.cheat_set_building_level(coord, id, level)
        .await
    })
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_infrastructure(app: AppHandle, coord: Coord) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_infrastructure(coord).await)
    .await?
    .map_err(Into::into)
}
