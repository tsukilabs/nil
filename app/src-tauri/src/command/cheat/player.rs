// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::cheat::player::*;
use nil_payload::response::cheat::player::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn cheat_get_player(
  app: AppHandle,
  req: CheatGetPlayerRequest,
) -> Result<CheatGetPlayerResponse> {
  app
    .client(async |cl| cl.cheat_get_player(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn cheat_get_players(
  app: AppHandle,
  req: CheatGetPlayersRequest,
) -> Result<CheatGetPlayersResponse> {
  app
    .client(async |cl| cl.cheat_get_players(req).await)
    .await
    .map_err(Into::into)
}
