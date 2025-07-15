// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::world::WorldOptions;
use std::net::SocketAddrV4;
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_server_addr(app: AppHandle) -> Result<SocketAddrV4> {
  app.client(async |cl| cl.server_addr()).await
}

#[tauri::command]
pub async fn get_server_version(app: AppHandle) -> Result<String> {
  app
    .client(async |cl| cl.version().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_server_ready(app: AppHandle) -> Result<bool> {
  app
    .client(async |cl| cl.is_ready().await)
    .await
}

#[tauri::command]
pub async fn start_server_with_options(
  app: AppHandle,
  world_options: WorldOptions,
) -> Result<SocketAddrV4> {
  app
    .nil()
    .start_server_with_options(world_options)
    .await
}

#[tauri::command]
pub async fn start_server_with_savedata(app: AppHandle, savedata: PathBuf) -> Result<SocketAddrV4> {
  app
    .nil()
    .start_server_with_savedata(savedata)
    .await
}

#[tauri::command]
pub async fn stop_server(app: AppHandle) {
  app.nil().stop_server().await;
}
