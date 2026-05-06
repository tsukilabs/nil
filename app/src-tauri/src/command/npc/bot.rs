// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_payload::request::npc::bot::*;
use nil_payload::response::npc::bot::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_bot_coords(
  app: AppHandle,
  req: GetBotCoordsRequest,
) -> Result<GetBotCoordsResponse> {
  app
    .client(async |cl| cl.get_bot_coords(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_bot(
  app: AppHandle,
  req: GetPublicBotRequest,
) -> Result<GetPublicBotResponse> {
  app
    .client(async |cl| cl.get_public_bot(req).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_bots(
  app: AppHandle,
  req: GetPublicBotsRequest,
) -> Result<GetPublicBotsResponse> {
  app
    .client(async |cl| cl.get_public_bots(req).await)
    .await
    .map_err(Into::into)
}
