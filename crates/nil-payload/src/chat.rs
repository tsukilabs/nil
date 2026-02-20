// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use jiff::Zoned;
use nil_core::chat::ChatHistory;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatHistoryRequest {
  pub world: WorldId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatHistoryResponse {
  pub public: ChatHistory,
  pub private: ChatHistory,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushChatMessageRequest {
  pub world: WorldId,
  pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushStdioMessagesRequest {
  pub world: WorldId,
  pub messages: Vec<PushStdioMessagesRequestMessage>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushStdioMessagesRequestMessage {
  pub content: String,
  #[serde(default)]
  pub time: Option<Zoned>,
}
