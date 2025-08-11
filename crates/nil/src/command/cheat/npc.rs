// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use nil_core::npc::bot::BotId;
use nil_core::npc::precursor::PrecursorId;
use nil_core::resources::Resources;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_bot_resources(app: AppHandle, id: BotId) -> Result<Resources> {
  app
    .client(async |cl| cl.cheat_get_bot_resources(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_bot_storage_capacity(
  app: AppHandle,
  id: BotId,
) -> Result<OverallStorageCapacity> {
  app
    .client(async |cl| cl.cheat_get_bot_storage_capacity(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_precursor_resources(app: AppHandle, id: PrecursorId) -> Result<Resources> {
  app
    .client(async |cl| cl.cheat_get_precursor_resources(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_precursor_storage_capacity(
  app: AppHandle,
  id: PrecursorId,
) -> Result<OverallStorageCapacity> {
  app
    .client(async |cl| {
      cl.cheat_get_precursor_storage_capacity(id)
        .await
    })
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_spawn_bot(app: AppHandle, name: String) -> Result<BotId> {
  app
    .client(async |cl| cl.cheat_spawn_bot(name).await)
    .await?
    .map_err(Into::into)
}
