// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Lua;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use tokio::fs;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
  pub name: String,
  pub chunk: String,
  pub path: PathBuf,
}

impl Script {
  pub async fn load(path: PathBuf) -> Result<Self> {
    let chunk = fs::read_to_string(&path).await?;
    let name = path
      .file_stem()
      .and_then(OsStr::to_str)
      .map(ToOwned::to_owned)
      .ok_or_else(|| io::ErrorKind::InvalidInput)?;

    Ok(Self { name, chunk, path })
  }

  pub async fn execute(self, lua: &mut Lua) -> Result<()> {
    lua.execute(&self.chunk).await
  }
}
