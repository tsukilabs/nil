// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::cheat::round::CheatSkipRoundRequest;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_skip_round(app: AppHandle, req: CheatSkipRoundRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_skip_round(req).await)
    .await?
    .map_err(Into::into)
}
