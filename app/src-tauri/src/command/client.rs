// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_client::ServerAddr;
use nil_core::player::PlayerId;
use nil_core::world::WorldId;
use nil_crypto::password::Password;
use nil_server_types::Token;
use tauri::AppHandle;

#[tauri::command]
pub fn get_client_version() -> &'static str {
  nil_client::VERSION
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) {
  app.nil().stop_client().await;
}

#[tauri::command]
pub async fn update_client(
  app: AppHandle,
  server_addr: ServerAddr,
  world_id: Option<WorldId>,
  world_password: Option<Password>,
  player_id: Option<PlayerId>,
  player_password: Option<Password>,
  authorization_token: Option<Token>,
) -> Result<()> {
  app
    .nil()
    .update_client(server_addr)
    .maybe_world_id(world_id)
    .maybe_world_password(world_password)
    .maybe_player_id(player_id)
    .maybe_player_password(player_password)
    .maybe_authorization_token(authorization_token)
    .call()
    .await
}
