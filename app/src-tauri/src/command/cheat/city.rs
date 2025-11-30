// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::cheat::city::CheatSetStabilityRequest;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_set_stability(app: AppHandle, req: CheatSetStabilityRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_stability(req).await)
    .await?
    .map_err(Into::into)
}
