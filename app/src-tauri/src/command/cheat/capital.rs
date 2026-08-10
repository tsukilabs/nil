// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::cheat::capital::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_set_influence(app: AppHandle, req: CheatSetInfluenceRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_influence(req).await)
    .await
    .map_err(Into::into)
}
