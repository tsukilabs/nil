// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::Coord;
use nil_core::player::{Player, PlayerId, PlayerOptions, PlayerStatus, PlayerStorageCapacity};
use nil_core::resource::Maintenance;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_player(app: AppHandle, id: PlayerId) -> Result<Player> {
  app
    .client(async |cl| cl.get_player(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_coords(app: AppHandle, id: PlayerId) -> Result<Vec<Coord>> {
  app
    .client(async |cl| cl.get_player_coords(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_maintenance(app: AppHandle) -> Result<Maintenance> {
  app
    .client(async |cl| cl.get_player_maintenance().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_status(app: AppHandle, id: PlayerId) -> Result<PlayerStatus> {
  app
    .client(async |cl| cl.get_player_status(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_storage_capacity(app: AppHandle) -> Result<PlayerStorageCapacity> {
  app
    .client(async |cl| cl.get_player_storage_capacity().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_players(app: AppHandle) -> Result<Vec<Player>> {
  app
    .client(async |cl| cl.get_players().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn player_exists(app: AppHandle, id: PlayerId) -> Result<bool> {
  app
    .client(async |cl| cl.player_exists(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn set_player_status(app: AppHandle, status: PlayerStatus) -> Result<()> {
  app
    .client(async |cl| cl.set_player_status(status).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn spawn_player(app: AppHandle, options: PlayerOptions) -> Result<()> {
  app
    .client(async |cl| cl.spawn_player(options).await)
    .await?
    .map_err(Into::into)
}
