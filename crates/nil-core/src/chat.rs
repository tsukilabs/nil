// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::player::PlayerId;
use derive_more::{Deref, DerefMut};
use jiff::Zoned;
use nil_num_macros::BigIntU64;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
  current_id: ChatMessageId,
  history: ChatHistory,
  size: NonZeroUsize,
}

impl Chat {
  pub const MIN_SIZE: NonZeroUsize = NonZeroUsize::new(100).unwrap();

  pub(crate) fn new(size: usize) -> Self {
    let size = size.max(Self::MIN_SIZE.get());
    let size = unsafe { NonZeroUsize::new_unchecked(size) };
    Self {
      current_id: ChatMessageId::default(),
      history: ChatHistory::default(),
      size,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &ChatMessage> {
    self.history.iter()
  }

  pub(crate) fn push(&mut self, message: &mut ChatMessage) -> ChatMessageId {
    self.current_id = self.current_id.next();
    *message.id_mut() = self.current_id;
    self.history.trim(self.size);
    self.history.push_back(message.clone());
    self.current_id
  }
}

impl Default for Chat {
  fn default() -> Self {
    Self::new(Self::MIN_SIZE.get())
  }
}

#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, Serialize)]
pub struct ChatHistory(VecDeque<ChatMessage>);

impl ChatHistory {
  fn trim(&mut self, max_size: NonZeroUsize) {
    let max_size = max_size.get();
    loop {
      let len = self.0.len();
      if len.saturating_sub(1) >= max_size {
        self.0.pop_front();
      } else {
        break;
      }
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ChatMessage {
  Player { message: ChatMessagePlayer },
}

impl ChatMessage {
  fn id_mut(&mut self) -> &mut ChatMessageId {
    match self {
      ChatMessage::Player { message } => &mut message.id,
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessagePlayer {
  id: ChatMessageId,
  author: PlayerId,
  content: ChatMessageContent,
  time: Zoned,
}

impl ChatMessagePlayer {
  pub fn new(author: PlayerId, message: &str) -> Self {
    Self {
      id: ChatMessageId::default(),
      author,
      content: ChatMessageContent::new(message),
      time: Zoned::now(),
    }
  }
}

impl From<ChatMessagePlayer> for ChatMessage {
  fn from(message: ChatMessagePlayer) -> Self {
    ChatMessage::Player { message }
  }
}

#[derive(Clone, Copy, Debug, Default, Deref, PartialEq, Eq, PartialOrd, Ord, Hash, BigIntU64)]
pub struct ChatMessageId(u64);

impl ChatMessageId {
  #[inline]
  #[must_use]
  const fn next(self) -> Self {
    Self(self.0.wrapping_add(1))
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
