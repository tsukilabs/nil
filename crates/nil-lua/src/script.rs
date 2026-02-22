// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Lua;
use crate::error::Result;
use crate::stdio::StdioMessage;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
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
      .unwrap_or_else(|| String::from("Script"));

    Ok(Self { name, chunk, path })
  }

  pub async fn execute(&self, lua: &mut Lua) -> Result<ScriptOutput> {
    lua.execute(&self.chunk).await
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptOutput {
  pub stdout: Vec<StdioMessage>,
  pub stderr: Vec<StdioMessage>,
}
