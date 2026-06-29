// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::player::*;
use nil_payload::response::player::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_player(app: AppHandle, req: GetPlayerRequest) -> Result<GetPlayerResponse> {
  app
    .client(async |cl| cl.get_player(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_coords(
  app: AppHandle,
  req: GetPlayerCoordsRequest,
) -> Result<GetPlayerCoordsResponse> {
  app
    .client(async |cl| cl.get_player_coords(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_ids(
  app: AppHandle,
  req: GetPlayerIdsRequest,
) -> Result<GetPlayerIdsResponse> {
  app
    .client(async |cl| cl.get_player_ids(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_maintenance(
  app: AppHandle,
  req: GetPlayerMaintenanceRequest,
) -> Result<GetPlayerMaintenanceResponse> {
  app
    .client(async |cl| cl.get_player_maintenance(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_military(
  app: AppHandle,
  req: GetPlayerMilitaryRequest,
) -> Result<GetPlayerMilitaryResponse> {
  app
    .client(async |cl| cl.get_player_military(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_status(
  app: AppHandle,
  req: GetPlayerStatusRequest,
) -> Result<GetPlayerStatusResponse> {
  app
    .client(async |cl| cl.get_player_status(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_storage_capacity(
  app: AppHandle,
  req: GetPlayerStorageCapacityRequest,
) -> Result<GetPlayerStorageCapacityResponse> {
  app
    .client(async |cl| cl.get_player_storage_capacity(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_worlds(
  app: AppHandle,
  req: GetPlayerWorldsRequest,
) -> Result<GetPlayerWorldsResponse> {
  app
    .client(async |cl| cl.get_player_worlds(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_player(
  app: AppHandle,
  req: GetPublicPlayerRequest,
) -> Result<GetPublicPlayerResponse> {
  app
    .client(async |cl| cl.get_public_player(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_players(
  app: AppHandle,
  req: GetPublicPlayersRequest,
) -> Result<GetPublicPlayersResponse> {
  app
    .client(async |cl| cl.get_public_players(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn player_exists(
  app: AppHandle,
  req: PlayerExistsRequest,
) -> Result<PlayerExistsResponse> {
  app
    .client(async |cl| cl.player_exists(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn set_player_status(app: AppHandle, req: SetPlayerStatusRequest) -> Result<()> {
  app
    .client(async |cl| cl.set_player_status(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn spawn_player(app: AppHandle, req: SpawnPlayerRequest) -> Result<()> {
  app
    .client(async |cl| cl.spawn_player(req).await)
    .await
    .map_err(Into::into)
}
