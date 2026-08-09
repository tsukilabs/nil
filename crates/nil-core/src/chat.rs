// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::player::PlayerId;
use bon::Builder;
use derive_more::From;
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::num::NonZeroU16;
use std::sync::Arc;
use strum::EnumIs;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[derive_const(Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
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
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ChatHistory {
  #[cfg_attr(feature = "typescript", ts(as = "Vec<ChatMessage>"))]
  queue: VecDeque<ChatMessage>,
  size: NonZeroU16,
}

impl ChatHistory {
  pub const MIN: NonZeroU16 = NonZeroU16::new(100).unwrap();
  pub const MAX: NonZeroU16 = NonZeroU16::new(500).unwrap();

  const fn new(size: u16) -> Self {
    let size = size.clamp(Self::MIN.get(), Self::MAX.get());
    let size = unsafe { NonZeroU16::new_unchecked(size) };
    Self { queue: VecDeque::new(), size }
  }

  fn push(&mut self, message: ChatMessage) {
    self.prune();
    self.queue.push_back(message);
  }

  fn prune(&mut self) {
    let len = self.queue.len();
    let size = usize::from(self.size.get());
    if len >= size {
      self.queue.drain(..=(len - size));
    }
  }
}

const impl Default for ChatHistory {
  fn default() -> Self {
    Self::new(Self::MIN.get())
  }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
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

#[derive(
  Clone, Copy, Debug, derive_more::Display, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
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

#[derive(Clone, Copy, Debug, EnumIs, Deserialize, Serialize)]
#[derive_const(Default)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum ChatMessageKind {
  #[default]
  Default,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum ChatMessageAuthor {
  #[default]
  System,
  Player {
    player: PlayerId,
  },
}

impl From<PlayerId> for ChatMessageAuthor {
  fn from(id: PlayerId) -> Self {
    Self::Player { player: id }
  }
}

impl From<&PlayerId> for ChatMessageAuthor {
  fn from(id: &PlayerId) -> Self {
    Self::Player { player: id.clone() }
  }
}

#[derive(Debug, From, Deserialize, Serialize)]
#[from(String, &str, Arc<str>, Box<str>, Cow<'_, str>)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ChatMessageContent(Arc<str>);

impl Clone for ChatMessageContent {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}
