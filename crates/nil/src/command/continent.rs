// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_continent_size(app: AppHandle) -> Result<usize> {
  app
    .client(async |cl| cl.get_continent_size().await)
    .await?
    .map_err(Into::into)
}
