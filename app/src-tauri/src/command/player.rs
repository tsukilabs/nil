// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use itertools::Itertools;
use nil_core::continent::Coord;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use nil_core::military::Military;
use nil_core::player::{Player, PlayerStatus, PublicPlayer};
use nil_core::report::ReportId;
use nil_core::resources::Maintenance;
use nil_payload::player::{
  GetPlayerCoordsRequest,
  GetPlayerRequest,
  GetPlayerStatusRequest,
  GetPublicPlayerRequest,
  PlayerExistsRequest,
  SetPlayerStatusRequest,
  SpawnPlayerRequest,
};
use nil_util::result::WrapOk;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_player(app: AppHandle, req: GetPlayerRequest) -> Result<Player> {
  app
    .client(async |cl| cl.get_player(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_coords(app: AppHandle, req: GetPlayerCoordsRequest) -> Result<Vec<Coord>> {
  app
    .client(async |cl| cl.get_player_coords(req).await)
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
pub async fn get_player_military(app: AppHandle) -> Result<Military> {
  app
    .client(async |cl| cl.get_player_military().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_reports(app: AppHandle) -> Result<Vec<ReportId>> {
  app
    .client(async |cl| cl.get_player_reports().await)
    .await??
    .into_iter()
    .unique()
    .sorted_unstable()
    .collect_vec()
    .wrap_ok()
}

#[tauri::command]
pub async fn get_player_status(
  app: AppHandle,
  req: GetPlayerStatusRequest,
) -> Result<PlayerStatus> {
  app
    .client(async |cl| cl.get_player_status(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_storage_capacity(app: AppHandle) -> Result<OverallStorageCapacity> {
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
pub async fn get_public_player(
  app: AppHandle,
  req: GetPublicPlayerRequest,
) -> Result<PublicPlayer> {
  app
    .client(async |cl| cl.get_public_player(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_players(app: AppHandle) -> Result<Vec<PublicPlayer>> {
  app
    .client(async |cl| cl.get_public_players().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn player_exists(app: AppHandle, req: PlayerExistsRequest) -> Result<bool> {
  app
    .client(async |cl| cl.player_exists(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn set_player_status(app: AppHandle, req: SetPlayerStatusRequest) -> Result<()> {
  app
    .client(async |cl| cl.set_player_status(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn spawn_player(app: AppHandle, req: SpawnPlayerRequest) -> Result<()> {
  app
    .client(async |cl| cl.spawn_player(req).await)
    .await?
    .map_err(Into::into)
}
