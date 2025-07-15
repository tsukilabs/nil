// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::player::PlayerId;
use serde::Serialize;
use serde::ser::Serializer;

pub use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Failed to connect websocket")]
  FailedToConnectWebsocket,
  #[error("Player name contains invalid characters: {0}")]
  InvalidPlayerId(PlayerId),
  #[error("{0}")]
  RequestFailed(String),

  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  Unknown(#[from] anyhow::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}
