// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::player::PlayerId;
use std::net::SocketAddrV4;
use tauri::AppHandle;

#[tauri::command]
pub async fn start_client(
  app: AppHandle,
  player_id: PlayerId,
  server_addr: SocketAddrV4,
) -> Result<()> {
  app
    .nil()
    .start_client(player_id, server_addr)
    .await
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) {
  app.nil().stop_client().await;
}
