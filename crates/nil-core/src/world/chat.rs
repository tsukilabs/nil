// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::chat::{ChatMessage, ChatMessageId};
use crate::player::PlayerId;

impl World {
  pub fn push_chat_message(&mut self, author: PlayerId, message: &str) -> ChatMessageId {
    let message = ChatMessage::new(author, message);
    let id = self.chat.push(message.clone());
    self.emit_chat_updated(message);
    id
  }
}
