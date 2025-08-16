// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::chat::ChatHistory;

impl Client {
  /// GET `/chat`
  pub async fn get_chat_history(&self) -> Result<(ChatHistory, ChatHistory)> {
    self.http.get_json("chat").await
  }

  /// POST `/chat`
  pub async fn push_chat_message(&self, message: String) -> Result<()> {
    self.http.post("chat", message).await
  }
}
