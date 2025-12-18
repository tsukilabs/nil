// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::chat::ChatHistory;
use nil_payload::chat::*;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_chat_history(
  app: AppHandle,
  req: GetChatHistoryRequest,
) -> Result<(ChatHistory, ChatHistory)> {
  app
    .client(async |cl| cl.get_chat_history(req).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn push_chat_message(app: AppHandle, req: PushChatMessageRequest) -> Result<()> {
  app
    .client(async |cl| cl.push_chat_message(req).await)
    .await?
    .map_err(Into::into)
}
