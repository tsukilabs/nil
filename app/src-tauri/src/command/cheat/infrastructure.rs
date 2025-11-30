// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::cheat::infrastructure::{
  CheatSetBuildingLevelRequest,
  CheatSetMaxInfrastructureRequest,
};
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_set_building_level(
  app: AppHandle,
  req: CheatSetBuildingLevelRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_building_level(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_max_infrastructure(
  app: AppHandle,
  req: CheatSetMaxInfrastructureRequest,
) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_max_infrastructure(req).await)
    .await?
    .map_err(Into::into)
}
