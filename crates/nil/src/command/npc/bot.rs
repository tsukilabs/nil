// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::Coord;
use nil_core::npc::bot::{BotId, PublicBot};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_bot_coords(app: AppHandle, id: BotId) -> Result<Vec<Coord>> {
  app
    .client(async |cl| cl.get_bot_coords(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_bot(app: AppHandle, id: BotId) -> Result<PublicBot> {
  app
    .client(async |cl| cl.get_public_bot(id).await)
    .await?
    .map_err(Into::into)
}
