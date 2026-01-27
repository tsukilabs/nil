// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::resources::Resources;
use nil_payload::cheat::resources::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_resources(
  app: AppHandle,
  req: CheatGetResourcesRequest,
) -> Result<Resources> {
  app
    .client(async |cl| cl.cheat_get_resources(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_food(app: AppHandle, req: CheatSetFoodRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_food(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_iron(app: AppHandle, req: CheatSetIronRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_iron(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_food(app: AppHandle, req: CheatSetMaxFoodRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_food(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_iron(app: AppHandle, req: CheatSetMaxIronRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_iron(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_resources(
  app: AppHandle,
  req: CheatSetMaxResourcesRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_resources(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_silo_resources(
  app: AppHandle,
  req: CheatSetMaxSiloResourcesRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_silo_resources(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_stone(app: AppHandle, req: CheatSetMaxStoneRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_stone(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_warehouse_resources(
  app: AppHandle,
  req: CheatSetMaxWarehouseResourcesRequest,
) -> Result<()> {
  app
    .client(async |cl| {
      cl.cheat_set_max_warehouse_resources(req)
        .await
    })
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_wood(app: AppHandle, req: CheatSetMaxWoodRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_wood(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_resources(app: AppHandle, req: CheatSetResourcesRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_resources(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_stone(app: AppHandle, req: CheatSetStoneRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_stone(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_wood(app: AppHandle, req: CheatSetWoodRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_wood(req).await)
    .await?
    .map_err(Into::into)
}
