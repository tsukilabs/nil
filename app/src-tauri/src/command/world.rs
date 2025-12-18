// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::manager::ManagerExt;
use nil_core::savedata::SavedataInfo;
use nil_core::world::{WorldConfig, WorldStats};
use nil_payload::world::*;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::async_runtime::spawn_blocking;

#[tauri::command]
pub async fn get_world_config(app: AppHandle, req: GetWorldConfigRequest) -> Result<WorldConfig> {
  app
    .client(async |cl| cl.get_world_config(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_world_stats(app: AppHandle, req: GetWorldStatsRequest) -> Result<WorldStats> {
  app
    .client(async |cl| cl.get_world_stats(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn read_savedata_info(path: PathBuf) -> Result<SavedataInfo> {
  spawn_blocking(move || SavedataInfo::read(&path))
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn save_world(app: AppHandle, req: SaveWorldRequest) -> Result<()> {
  if app.nil().is_host().await {
    app
      .client(async |cl| cl.save_world(req).await)
      .await?
      .map_err(Into::into)
  } else {
    Err(Error::Forbidden)
  }
}
