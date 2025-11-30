// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::Serialize;
use serde::ser::Serializer;
use std::error::Error as StdError;

use std::io;
pub use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;
pub type BoxResult<T> = StdResult<T, Box<dyn StdError>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Client not connected")]
  ClientNotConnected,
  #[error("Only the server host is authorized to execute this action")]
  Forbidden,

  #[error(transparent)]
  Client(#[from] nil_client::Error),
  #[error(transparent)]
  Core(#[from] nil_core::error::Error),
  #[error(transparent)]
  Server(#[from] nil_server::Error),

  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
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

impl From<io::ErrorKind> for Error {
  fn from(value: io::ErrorKind) -> Self {
    Self::Io(io::Error::from(value))
  }
}
