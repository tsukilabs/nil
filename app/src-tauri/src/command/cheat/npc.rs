// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::npc::bot::BotId;
use nil_core::npc::precursor::PrecursorId;
use nil_core::resources::Resources;
use nil_payload::cheat::npc::CheatSpawnBotRequest;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_bot_resources(app: AppHandle, id: BotId) -> Result<Resources> {
  app
    .client(async |cl| cl.cheat_get_bot_resources(id).await)
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
pub async fn cheat_spawn_bot(app: AppHandle, req: CheatSpawnBotRequest) -> Result<BotId> {
  app
    .client(async |cl| cl.cheat_spawn_bot(req).await)
    .await?
    .map_err(Into::into)
}
