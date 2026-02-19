// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::player::Player;
use nil_payload::cheat::player::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_players(app: AppHandle, req: CheatGetPlayersRequest) -> Result<Vec<Player>> {
  app
    .client(async |cl| cl.cheat_get_players(req).await)
    .await
    .map_err(Into::into)
}
