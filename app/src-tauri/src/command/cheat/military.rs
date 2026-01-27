// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::cheat::military::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_spawn_personnel(app: AppHandle, req: CheatSpawnPersonnelRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_spawn_personnel(req).await)
    .await?
    .map_err(Into::into)
}
