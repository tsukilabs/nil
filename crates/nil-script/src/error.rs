// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::Serialize;
use serde::ser::Serializer;
use std::convert::Infallible;
use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("No line found for offset {offset}")]
  LineNotFound { offset: usize },

  #[error("Too many constants in one chunk")]
  TooManyConstants,
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
