// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_client::ClientOptions;
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
pub async fn update_client(app: AppHandle, options: ClientOptions) -> Result<()> {
  app.nil().update_client(options).await
}
