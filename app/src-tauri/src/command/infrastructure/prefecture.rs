// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::infrastructure::prefecture::*;
use nil_payload::response::infrastructure::prefecture::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn add_prefecture_build_order(
  app: AppHandle,
  req: AddPrefectureBuildOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.add_prefecture_build_order(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cancel_prefecture_build_order(
  app: AppHandle,
  req: CancelPrefectureBuildOrderRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cancel_prefecture_build_order(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_prefecture_build_catalog(
  app: AppHandle,
  req: GetPrefectureBuildCatalogRequest,
) -> Result<GetPrefectureBuildCatalogResponse> {
  app
    .client(async |cl| cl.get_prefecture_build_catalog(req).await)
    .await
    .map_err(Into::into)
}
