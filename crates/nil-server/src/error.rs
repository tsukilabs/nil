// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::world::config::WorldId;
use serde::Serialize;
use serde::ser::Serializer;
use std::convert::Infallible;
use std::io;
use std::result::Result as StdResult;

pub use nil_core::error::Error as CoreError;
pub use nil_database::error::Error as DatabaseError;

pub type Result<T, E = Error> = StdResult<T, E>;
pub type AnyResult<T> = anyhow::Result<T>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Failed to start server")]
  FailedToStart,

  #[error("Incorrect username or password")]
  IncorrectUserCredentials,

  #[error("Incorrect world password")]
  IncorrectWorldCredentials(WorldId),

  #[error("Invalid world")]
  InvalidWorld(WorldId),

  #[error("Missing password")]
  MissingPassword,

  #[error(transparent)]
  Core(#[from] CoreError),
  #[error(transparent)]
  Database(#[from] DatabaseError),
  #[error(transparent)]
  Io(#[from] io::Error),
  #[error(transparent)]
  Semver(#[from] semver::Error),
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

impl<E> From<Result<Infallible, E>> for Error
where
  E: Into<Error>,
{
  fn from(value: Result<Infallible, E>) -> Self {
    value.unwrap_err().into()
  }
}

impl From<io::ErrorKind> for Error {
  fn from(value: io::ErrorKind) -> Self {
    Self::Io(io::Error::from(value))
  }
}
