// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::round::Round;
use tauri::AppHandle;

#[tauri::command]
pub async fn end_turn(app: AppHandle) -> Result<()> {
  app
    .client(async |cl| cl.end_turn().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_round(app: AppHandle) -> Result<Round> {
  app
    .client(async |cl| cl.get_round().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_round_idle(app: AppHandle) -> Result<bool> {
  app
    .client(async |cl| cl.get_round().await)
    .await?
    .map(|round| round.is_idle())
    .map_err(Into::into)
}

#[tauri::command]
pub async fn start_round(app: AppHandle) -> Result<()> {
  if app.nil().is_host().await {
    app
      .client(async |cl| cl.start_round().await)
      .await??;
  }

  Ok(())
}
