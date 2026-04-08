// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::player::PlayerId;
use bon::Builder;
use derive_more::From;
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::Arc;
use strum::EnumIs;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
  history: ChatHistory,
}

impl Chat {
  #[inline]
  pub fn history(&self) -> ChatHistory {
    self.history.clone()
  }

  pub(crate) fn push(&mut self, message: ChatMessage) {
    self.history.push(message);
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

  fn push(&mut self, message: ChatMessage) {
    self.prune();
    self.queue.push_back(message);
  }

  fn prune(&mut self) {
    let size = self.size.get();
    let len = self.queue.len();
    if len >= size {
      self.queue.drain(..=(len - size));
    }
  }
}

impl Default for ChatHistory {
  fn default() -> Self {
    Self::new(Self::MIN.get())
  }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
  #[builder(start_fn, into)]
  content: ChatMessageContent,

  #[builder(skip)]
  id: ChatMessageId,

  #[builder(default)]
  kind: ChatMessageKind,

  #[builder(default, into)]
  author: ChatMessageAuthor,

  #[builder(default = Zoned::now())]
  time: Zoned,
}

impl ChatMessage {
  #[inline]
  pub fn id(&self) -> ChatMessageId {
    self.id
  }

  #[inline]
  pub fn kind(&self) -> ChatMessageKind {
    self.kind
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

#[derive(Clone, Copy, Debug, Default, EnumIs, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChatMessageKind {
  #[default]
  Default,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ChatMessageAuthor {
  #[default]
  System,
  Player {
    id: PlayerId,
  },
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

#[derive(Debug, From, Deserialize, Serialize)]
#[from(String, &str, Arc<str>, Box<str>, Cow<'_, str>)]
pub struct ChatMessageContent(Arc<str>);

impl Clone for ChatMessageContent {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}
