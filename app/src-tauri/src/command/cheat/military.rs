// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::cheat::military::*;
use nil_payload::response::cheat::military::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_idle_armies_at(
  app: AppHandle,
  req: CheatGetIdleArmiesAtRequest,
) -> Result<CheatGetIdleArmiesAtResponse> {
  app
    .client(async |cl| cl.cheat_get_idle_armies_at(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_idle_personnel_at(
  app: AppHandle,
  req: CheatGetIdlePersonnelAtRequest,
) -> Result<CheatGetIdlePersonnelAtResponse> {
  app
    .client(async |cl| cl.cheat_get_idle_personnel_at(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_maneuvers(
  app: AppHandle,
  req: CheatGetManeuversRequest,
) -> Result<CheatGetManeuversResponse> {
  app
    .client(async |cl| cl.cheat_get_maneuvers(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_maneuvers_of(
  app: AppHandle,
  req: CheatGetManeuversOfRequest,
) -> Result<CheatGetManeuversOfResponse> {
  app
    .client(async |cl| cl.cheat_get_maneuvers_of(req).await)
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
