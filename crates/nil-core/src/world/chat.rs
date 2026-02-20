// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::chat::{ChatMessage, ChatMessageAuthor, ChatMessageKind};
use crate::player::PlayerId;
use crate::world::World;
use itertools::Itertools;
use jiff::Zoned;

impl World {
  pub fn push_chat_message(&mut self, author: PlayerId, message: &str) {
    let message = ChatMessage::builder(message)
      .author(author)
      .kind(ChatMessageKind::Default)
      .build();

    self.chat.push(message.clone());
    self.emit_chat_updated(vec![message]);
  }

  pub fn push_stdio_messages<I>(&mut self, player: PlayerId, iter: I)
  where
    I: IntoIterator<Item = (String, Option<Zoned>)>,
  {
    let mut messages = Vec::new();
    let now = Zoned::now();

    for (content, time) in iter
      .into_iter()
      .filter(|(content, _)| !content.is_empty())
      .sorted_by(|(_, time_a), (_, time_b)| {
        let time_a = time_a.as_ref().unwrap_or(&now);
        let time_b = time_b.as_ref().unwrap_or(&now);
        time_a.cmp(time_b)
      })
    {
      let message = ChatMessage::builder(content)
        .author(ChatMessageAuthor::System)
        .kind(ChatMessageKind::Stdio)
        .maybe_time(time)
        .build();

      self
        .chat
        .push_to(player.clone(), message.clone());

      messages.push(message);
    }

    self.emit_chat_updated_to(player, messages);
  }
}
