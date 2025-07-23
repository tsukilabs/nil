// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::player::PlayerId;
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
  history: ChatHistory,
}

impl Chat {
  pub fn iter(&self) -> impl Iterator<Item = &ChatMessage> {
    self.history.queue.iter()
  }

  pub(crate) fn push(&mut self, message: ChatMessage) -> ChatMessageId {
    let id = message.id;
    self.history.trim();
    self.history.queue.push_back(message);
    id
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatHistory {
  queue: VecDeque<ChatMessage>,
  size: NonZeroUsize,
}

impl ChatHistory {
  pub const MIN: NonZeroUsize = NonZeroUsize::new(100).unwrap();
  pub const MAX: NonZeroUsize = NonZeroUsize::new(500).unwrap();

  fn new(size: usize) -> Self {
    let size = size.clamp(Self::MIN.get(), Self::MAX.get());
    let size = unsafe { NonZeroUsize::new_unchecked(size) };
    Self { queue: VecDeque::new(), size }
  }

  fn trim(&mut self) {
    let size = self.size.get();
    loop {
      let len = self.queue.len();
      if len.saturating_sub(1) >= size {
        self.queue.pop_front();
      } else {
        break;
      }
    }
  }
}

impl Default for ChatHistory {
  fn default() -> Self {
    Self::new(Self::MIN.get())
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
  id: ChatMessageId,
  author: ChatMessageAuthor,
  content: ChatMessageContent,
  timestamp: Zoned,
}

impl ChatMessage {
  pub fn new<A>(author: A, message: &str) -> Self
  where
    A: Into<ChatMessageAuthor>,
  {
    Self {
      id: ChatMessageId::new(),
      author: author.into(),
      content: ChatMessageContent::new(message),
      timestamp: Zoned::now(),
    }
  }
}

impl From<ChatMessage> for ChatMessageAuthor {
  fn from(message: ChatMessage) -> Self {
    message.author
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatMessageId(Uuid);

impl ChatMessageId {
  #[must_use]
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
}

impl Default for ChatMessageId {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ChatMessageAuthor {
  Player { id: PlayerId },
}

impl From<PlayerId> for ChatMessageAuthor {
  fn from(id: PlayerId) -> Self {
    Self::Player { id }
  }
}

impl From<&PlayerId> for ChatMessageAuthor {
  fn from(id: &PlayerId) -> Self {
    Self::Player { id: id.clone() }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessageContent(Arc<str>);

impl ChatMessageContent {
  pub fn new(content: &str) -> Self {
    Self(Arc::from(content))
  }
}

impl Clone for ChatMessageContent {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}
