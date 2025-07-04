// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::building::prefecture::{
  PrefectureBuildOrderOptions,
  PrefectureCatalog,
};
use nil_core::village::Coord;
use tauri::AppHandle;

#[tauri::command]
pub async fn add_prefecture_build_order(
  app: AppHandle,
  options: PrefectureBuildOrderOptions,
) -> Result<()> {
  app
    .client(async |cl| cl.add_prefecture_build_order(options).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_prefecture_catalog(app: AppHandle, coord: Coord) -> Result<PrefectureCatalog> {
  app
    .client(async |cl| cl.get_prefecture_catalog(coord).await)
    .await?
    .map_err(Into::into)
}
