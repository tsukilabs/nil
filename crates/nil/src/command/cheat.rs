// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::resource::{Food, Iron, Resources, Stone, Wood};
use nil_core::village::{Coord, Stability};
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_set_food(app: AppHandle, food: Food) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_food(food).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_iron(app: AppHandle, iron: Iron) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_iron(iron).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_resources(app: AppHandle) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_resources().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_resources(app: AppHandle, resources: Resources) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_resources(resources).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_stability(app: AppHandle, coord: Coord, stability: Stability) -> Result<()> {
  app
    .client(async |cl| {
      cl.cheat_set_stability(coord, stability)
        .await
    })
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_stone(app: AppHandle, stone: Stone) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_stone(stone).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_wood(app: AppHandle, wood: Wood) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_wood(wood).await)
    .await?
    .map_err(Into::into)
}
