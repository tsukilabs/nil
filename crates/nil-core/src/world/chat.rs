// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::{ChatMessage, ChatMessageId, ChatMessageKind};
use crate::player::PlayerId;
use crate::world::World;

impl World {
  pub fn push_chat_message(&mut self, author: PlayerId, message: &str) -> ChatMessageId {
    let message = ChatMessage::builder(message)
      .author(author)
      .kind(ChatMessageKind::Default)
      .build();

    let id = message.id();
    self.chat.push(message.clone());
    self.emit_chat_updated(message);

    id
  }
}
