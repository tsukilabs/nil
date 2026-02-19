// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::chat::*;

impl Client {
  pub async fn get_chat_history(
    &self,
    req: GetChatHistoryRequest,
  ) -> Result<GetChatHistoryResponse> {
    http::json_post("get-chat-history")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn push_chat_message(&self, req: PushChatMessageRequest) -> Result<()> {
    http::post("push-chat-message")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn push_stdout_message(&self, req: PushStdoutMessageRequest) -> Result<()> {
    http::post("push-stdout-message")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
