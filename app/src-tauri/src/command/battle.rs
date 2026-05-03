// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::battle::*;
use nil_payload::response::battle::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn simulate_battle(
  app: AppHandle,
  req: SimulateBattleRequest,
) -> Result<SimulateBattleResponse> {
  app
    .client(async |cl| cl.simulate_battle(req).await)
    .await
    .map_err(Into::into)
}
