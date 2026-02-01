// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::military::army::{Army, ArmyPersonnel};
use nil_payload::cheat::military::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_idle_armies_at(
  app: AppHandle,
  req: CheatGetIdleArmiesAtRequest,
) -> Result<Vec<Army>> {
  app
    .client(async |cl| cl.cheat_get_idle_armies_at(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_idle_personnel_at(
  app: AppHandle,
  req: CheatGetIdlePersonnelAtRequest,
) -> Result<ArmyPersonnel> {
  app
    .client(async |cl| cl.cheat_get_idle_personnel_at(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_spawn_personnel(app: AppHandle, req: CheatSpawnPersonnelRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_spawn_personnel(req).await)
    .await
    .map_err(Into::into)
}
