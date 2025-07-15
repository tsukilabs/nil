// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::building::prefecture::{
  PrefectureBuildCatalog,
  PrefectureBuildOrderRequest,
};
use nil_core::village::Coord;
use tauri::AppHandle;

#[tauri::command]
pub async fn add_prefecture_build_order(
  app: AppHandle,
  request: PrefectureBuildOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.add_prefecture_build_order(request).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cancel_prefecture_build_order(app: AppHandle, coord: Coord) -> Result<()> {
  app
    .client(async |cl| cl.cancel_prefecture_build_order(coord).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_prefecture_build_catalog(
  app: AppHandle,
  coord: Coord,
) -> Result<PrefectureBuildCatalog> {
  app
    .client(async |cl| cl.get_prefecture_build_catalog(coord).await)
    .await?
    .map_err(Into::into)
}
