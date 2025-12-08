// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::continent::Coord;
use nil_core::npc::bot::PublicBot;
use nil_payload::npc::bot::{GetBotCoordsRequest, GetPublicBotRequest};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_bot_coords(app: AppHandle, req: GetBotCoordsRequest) -> Result<Vec<Coord>> {
  app
    .client(async |cl| cl.get_bot_coords(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_public_bot(app: AppHandle, req: GetPublicBotRequest) -> Result<PublicBot> {
  app
    .client(async |cl| cl.get_public_bot(req).await)
    .await?
    .map_err(Into::into)
}
