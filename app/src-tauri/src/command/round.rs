// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::manager::ManagerExt;
use nil_core::round::Round;
use nil_payload::round::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_round(app: AppHandle, req: GetRoundRequest) -> Result<Round> {
  app
    .client(async |cl| cl.get_round(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_round_idle(app: AppHandle, req: GetRoundRequest) -> Result<bool> {
  app
    .client(async |cl| cl.get_round(req).await)
    .await?
    .map(|round| round.is_idle())
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_round_waiting(app: AppHandle, req: GetRoundRequest) -> Result<bool> {
  app
    .client(async |cl| cl.get_round(req).await)
    .await?
    .map(|round| round.is_waiting())
    .map_err(Into::into)
}

#[tauri::command]
pub async fn set_player_ready(app: AppHandle, req: SetPlayerReadyRequest) -> Result<()> {
  app
    .client(async |cl| cl.set_player_ready(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn start_round(app: AppHandle, req: StartRoundRequest) -> Result<()> {
  if app.nil().is_host().await {
    app
      .client(async |cl| cl.start_round(req).await)
      .await?
      .map_err(Into::into)
  } else {
    Err(Error::Forbidden)
  }
}
