// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::ethic::Ethics;
use nil_core::npc::bot::BotId;
use nil_payload::cheat::npc::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_ethics(
  app: AppHandle,
  req: CheatGetEthicsRequest,
) -> Result<Option<Ethics>> {
  app
    .client(async |cl| cl.cheat_get_ethics(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_set_bot_ethics(app: AppHandle, req: CheatSetBotEthicsRequest) -> Result<()> {
  app
    .client(async |cl| cl.cheat_set_bot_ethics(req).await)
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
