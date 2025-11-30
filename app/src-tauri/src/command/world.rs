// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::manager::ManagerExt;
use nil_core::world::{WorldConfig, WorldStats};
use nil_payload::world::SaveWorldRequest;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_world_config(app: AppHandle) -> Result<WorldConfig> {
  app
    .client(async |cl| cl.get_world_config().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_world_stats(app: AppHandle) -> Result<WorldStats> {
  app
    .client(async |cl| cl.get_world_stats().await)
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
