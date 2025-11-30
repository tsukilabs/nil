// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod prefecture;
pub mod stable;

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::infrastructure::ToggleBuildingRequest;
use tauri::AppHandle;

#[tauri::command]
pub async fn toggle_building(app: AppHandle, req: ToggleBuildingRequest) -> Result<()> {
  app
    .client(async |cl| cl.toggle_building(req).await)
    .await?
    .map_err(Into::into)
}
