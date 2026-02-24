// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::Serialize;
use serde::ser::Serializer;
use std::io;

pub use mlua::Result as LuaResult;
pub use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;
pub type AnyResult<T> = anyhow::Result<T>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Client(#[from] nil_client::Error),
  #[error(transparent)]
  Io(#[from] io::Error),
  #[error(transparent)]
  Lua(#[from] mlua::Error),
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
