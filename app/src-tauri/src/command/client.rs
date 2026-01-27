// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_client::ServerAddr;
use nil_core::player::PlayerId;
use nil_core::world::WorldId;
use nil_util::password::Password;
use tauri::AppHandle;

#[tauri::command]
pub async fn start_client(
  app: AppHandle,
  server_addr: ServerAddr,
  world_id: Option<WorldId>,
  player_id: PlayerId,
  password: Option<Password>,
) -> Result<()> {
  app
    .nil()
    .start_client(server_addr, world_id, player_id, password)
    .await
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) {
  app.nil().stop_client().await;
}
