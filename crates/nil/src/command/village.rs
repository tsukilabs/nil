// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::Coord;
use nil_core::village::Village;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_village(app: AppHandle, coord: Coord) -> Result<Village> {
  app
    .client(async |cl| cl.get_village(coord).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn rename_village(app: AppHandle, coord: Coord, name: String) -> Result<()> {
  app
    .client(async |cl| cl.rename_village(coord, &name).await)
    .await?
    .map_err(Into::into)
}
