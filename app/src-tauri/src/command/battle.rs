// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::battle::BattleResult;
use nil_payload::battle::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn simulate_battle(app: AppHandle, req: SimulateBattleRequest) -> Result<BattleResult> {
  app
    .client(async |cl| cl.simulate_battle(req).await)
    .await?
    .map_err(Into::into)
}
