// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_client::ServerAddr;
use nil_core::player::PlayerId;
use nil_core::world::WorldId;
use nil_server_types::Token;
use nil_util::password::Password;
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
  player_id: Option<PlayerId>,
  player_password: Option<Password>,
  authorization_token: Option<Token>,
) -> Result<()> {
  app
    .nil()
    .update_client(
      server_addr,
      world_id,
      player_id,
      player_password,
      authorization_token,
    )
    .await
}
