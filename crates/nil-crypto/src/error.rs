// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::Serialize;
use serde::ser::Serializer;

pub use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;
pub type AnyResult<T> = anyhow::Result<T>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  PasswordHash(#[from] argon2::password_hash::Error),
  #[error(transparent)]
  Phc(#[from] argon2::password_hash::phc::Error),
  #[error(transparent)]
  Unknown(#[from] anyhow::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}
