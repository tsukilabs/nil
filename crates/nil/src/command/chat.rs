// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::chat::{Chat, ChatMessageId};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_chat(app: AppHandle) -> Result<Chat> {
  app
    .client(async |cl| cl.get_chat().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn push_chat_message(app: AppHandle, message: String) -> Result<ChatMessageId> {
  app
    .client(async |cl| cl.push_chat_message(message).await)
    .await?
    .map_err(Into::into)
}
