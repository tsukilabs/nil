// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use jiff::Zoned;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use tokio::sync::mpsc::UnboundedReceiver;

static ID: AtomicU32 = AtomicU32::new(1);

pub struct Stdio {
  pub(crate) buffer: Vec<StdioMessage>,
  receiver: UnboundedReceiver<StdioMessage>,
}

impl Stdio {
  pub(crate) fn new(receiver: UnboundedReceiver<StdioMessage>) -> Self {
    Self { buffer: Vec::new(), receiver }
  }

  pub(crate) fn flush(&mut self) {
    while let Ok(message) = self.receiver.try_recv() {
      self.buffer.push(message);
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdioMessage {
  id: u32,
  content: String,
  time: Zoned,
}

impl StdioMessage {
  pub(crate) fn new(content: String) -> Self {
    Self {
      id: ID.fetch_add(1, Relaxed),
      content,
      time: Zoned::now(),
    }
  }
}

impl PartialOrd for StdioMessage {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for StdioMessage {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .time
      .cmp(&other.time)
      .then_with(|| self.id.cmp(&other.id))
  }
}

impl fmt::Display for StdioMessage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.content)
  }
}
