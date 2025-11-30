// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::{Coord, PublicField};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_field(app: AppHandle, coord: Coord) -> Result<PublicField> {
  app
    .client(async |cl| cl.get_field(coord).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_fields(app: AppHandle, coords: Vec<Coord>) -> Result<Vec<(Coord, PublicField)>> {
  if coords.is_empty() {
    return Ok(Vec::new());
  }

  app
    .client(async |cl| cl.get_fields(coords).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_continent_size(app: AppHandle) -> Result<usize> {
  app
    .client(async |cl| cl.get_continent_size().await)
    .await?
    .map_err(Into::into)
}
