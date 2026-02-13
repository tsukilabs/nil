// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{CoreError, Error, Result};
use crate::manager::ManagerExt;
use itertools::Itertools;
use nil_core::npc::bot::BotId;
use nil_core::npc::precursor::PrecursorId;
use nil_core::player::{Player, PlayerId};
use nil_core::savedata::{Savedata, SavedataInfo};
use nil_core::world::config::{WorldConfig, WorldId};
use nil_core::world::stats::WorldStats;
use nil_payload::world::*;
use nil_server_types::RemoteWorld;
use std::path::PathBuf;
use tap::Pipe;
use tauri::AppHandle;
use tauri::async_runtime::spawn_blocking;

#[tauri::command]
pub async fn create_remote_world(app: AppHandle, req: CreateRemoteWorldRequest) -> Result<WorldId> {
  app
    .client(async |cl| cl.create_remote_world(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_remote_world(app: AppHandle, req: GetRemoteWorldRequest) -> Result<RemoteWorld> {
  app
    .client(async |cl| cl.get_remote_world(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_remote_worlds(app: AppHandle) -> Result<Vec<RemoteWorld>> {
  app
    .client(async |cl| cl.get_remote_worlds().await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_savedata_players(path: PathBuf) -> Result<Vec<PlayerId>> {
  let bytes = tokio::fs::read(path).await?;
  spawn_blocking(move || {
    Savedata::read(&bytes)?
      .players()
      .map(Player::id)
      .collect_vec()
      .pipe(Ok::<_, CoreError>)
  })
  .await?
  .map_err(Into::into)
}

#[tauri::command]
pub async fn get_world_bots(app: AppHandle, req: GetWorldBotsRequest) -> Result<Vec<BotId>> {
  app
    .client(async |cl| cl.get_world_bots(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_world_config(app: AppHandle, req: GetWorldConfigRequest) -> Result<WorldConfig> {
  app
    .client(async |cl| cl.get_world_config(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_world_players(
  app: AppHandle,
  req: GetWorldPlayersRequest,
) -> Result<Vec<PlayerId>> {
  app
    .client(async |cl| cl.get_world_players(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_world_precursors(
  app: AppHandle,
  req: GetWorldPrecursorsRequest,
) -> Result<Vec<PrecursorId>> {
  app
    .client(async |cl| cl.get_world_precursors(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_world_stats(app: AppHandle, req: GetWorldStatsRequest) -> Result<WorldStats> {
  app
    .client(async |cl| cl.get_world_stats(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn read_savedata_info(path: PathBuf) -> Result<SavedataInfo> {
  let bytes = tokio::fs::read(path).await?;
  spawn_blocking(move || SavedataInfo::read(&bytes))
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn save_local_world(app: AppHandle, req: SaveLocalWorldRequest) -> Result<()> {
  if app.nil().is_local_and_host().await {
    app
      .client(async |cl| cl.save_local_world(req).await)
      .await
      .map_err(Into::into)
  } else {
    Err(Error::Forbidden)
  }
}
