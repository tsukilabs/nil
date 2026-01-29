// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_client::ServerAddr;
use nil_core::player::PlayerId;
use nil_core::world::WorldOptions;
use nil_payload::{AuthorizeRequest, ValidateTokenRequest};
use nil_server::local::LocalServer;
use nil_server_types::{ServerKind, Token};
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub async fn authorize(app: AppHandle, req: AuthorizeRequest) -> Result<Token> {
  app
    .client(async |cl| cl.authorize(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_server_addr(app: AppHandle) -> ServerAddr {
  app.client(async |cl| cl.server_addr()).await
}

#[tauri::command]
pub async fn get_server_kind(app: AppHandle) -> Result<ServerKind> {
  app
    .client(async |cl| cl.get_server_kind().await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_server_version(app: AppHandle) -> Result<String> {
  app
    .client(async |cl| cl.get_server_version().await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_server_ready(app: AppHandle) -> bool {
  app
    .client(async |cl| cl.is_ready().await)
    .await
}

#[tauri::command]
pub async fn start_server_with_options(
  app: AppHandle,
  world_options: WorldOptions,
) -> Result<LocalServer> {
  app
    .nil()
    .start_server_with_options(world_options)
    .await
}

#[tauri::command]
pub async fn start_server_with_savedata(app: AppHandle, savedata: PathBuf) -> Result<LocalServer> {
  app
    .nil()
    .start_server_with_savedata(savedata)
    .await
}

#[tauri::command]
pub async fn stop_server(app: AppHandle) {
  app.nil().stop_server().await;
}

#[tauri::command]
pub async fn validate_token(app: AppHandle, req: ValidateTokenRequest) -> Result<Option<PlayerId>> {
  app
    .client(async |cl| cl.validate_token(req).await)
    .await
    .map_err(Into::into)
}
