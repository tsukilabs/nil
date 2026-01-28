// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::sql_types::id::UserDataId;
use crate::sql_types::user::User;
use either::Either;
use nil_core::world::WorldId;
use serde::Serialize;
use serde::ser::Serializer;
use std::convert::Infallible;
use std::result::Result as StdResult;

pub use nil_core::error::Error as CoreError;

pub type Result<T, E = Error> = StdResult<T, E>;
pub type AnyResult<T> = anyhow::Result<T>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Invalid password")]
  InvalidPassword,

  #[error("Invalid username: \"{0}\"")]
  InvalidUsername(User),

  #[error("User already exists: \"{0}\"")]
  UserAlreadyExists(User),

  #[error("User not found")]
  UserNotFound(Either<User, UserDataId>),

  #[error("World not found")]
  WorldNotFound(WorldId),

  #[error(transparent)]
  Core(#[from] CoreError),
  #[error(transparent)]
  Diesel(#[from] diesel::result::Error),
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
