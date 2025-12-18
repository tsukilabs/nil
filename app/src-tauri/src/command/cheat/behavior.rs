// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::behavior::build::BuildStep;
use nil_payload::cheat::behavior::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_build_steps(
  app: AppHandle,
  req: CheatGetBuildStepsRequest,
) -> Result<Vec<BuildStep>> {
  app
    .client(async |cl| cl.cheat_get_build_steps(req).await)
    .await?
    .map_err(Into::into)
}
