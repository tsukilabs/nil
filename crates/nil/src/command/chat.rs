// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::chat::ChatHistory;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_chat_history(app: AppHandle) -> Result<(ChatHistory, ChatHistory)> {
  app
    .client(async |cl| cl.get_chat_history().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn push_chat_message(app: AppHandle, message: String) -> Result<()> {
  app
    .client(async |cl| cl.push_chat_message(message).await)
    .await?
    .map_err(Into::into)
}
