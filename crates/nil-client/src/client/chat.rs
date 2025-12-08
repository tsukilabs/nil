// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::chat::ChatHistory;
use nil_payload::chat::PushChatMessageRequest;

impl Client {
  pub async fn get_chat_history(&self) -> Result<(ChatHistory, ChatHistory)> {
    self.http.json_get("get-chat-history").await
  }

  pub async fn push_chat_message(&self, req: PushChatMessageRequest) -> Result<()> {
    self
      .http
      .post("push-chat-message", req)
      .await
  }
}
