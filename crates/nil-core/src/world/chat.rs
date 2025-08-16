// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::chat::{ChatMessage, ChatMessageAuthor, ChatMessageKind};
use crate::player::PlayerId;
use crate::script::Stdout;

impl World {
  pub fn push_chat_message(&mut self, author: PlayerId, message: &str) {
    let message = ChatMessage::builder(message)
      .author(author)
      .kind(ChatMessageKind::Default)
      .build();

    self.chat.push(message.clone());
    self.emit_chat_updated(message);
  }

  pub(crate) fn push_stdout_message(&mut self, player: PlayerId, stdout: Stdout) {
    let message = ChatMessage::builder(stdout.to_string())
      .author(ChatMessageAuthor::System)
      .kind(ChatMessageKind::Stdout)
      .build();

    self
      .chat
      .push_to(player.clone(), message.clone());

    self.emit_chat_updated_to(player, message);
  }
}
