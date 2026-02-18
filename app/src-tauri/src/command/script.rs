// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::manager::ManagerExt;
use futures::TryFutureExt;
use nil_lua::ScriptOutput;
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub async fn execute_script(app: AppHandle, chunk: String) -> Result<ScriptOutput> {
  app
    .lua(async |lua| lua.execute(&chunk).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn execute_script_at(app: AppHandle, path: PathBuf) -> Result<ScriptOutput> {
  tokio::fs::read_to_string(path)
    .map_err(Into::<Error>::into)
    .and_then(|chunk| execute_script(app, chunk))
    .await
}
