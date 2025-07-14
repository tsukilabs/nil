// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::manager::ManagerExt;
use nil_core::script::{Script, ScriptId};
use std::ffi::OsStr;
use std::io::ErrorKind::NotADirectory;
use std::path::PathBuf;
use tauri::AppHandle;
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};

#[tauri::command]
pub async fn add_scripts(app: AppHandle, scripts: Vec<Script>) -> Result<Vec<ScriptId>> {
  if scripts.is_empty() {
    Ok(Vec::new())
  } else {
    app
      .client(async |cl| cl.add_scripts(scripts).await)
      .await?
      .map_err(Into::into)
  }
}

#[tauri::command]
pub async fn execute_script(app: AppHandle, id: ScriptId) -> Result<()> {
  app
    .client(async |cl| cl.execute_script(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn export_script(dir: PathBuf, script: Script) -> Result<()> {
  if dir.is_dir() {
    let mut path = dir.join(&script.name);
    path.set_extension(Script::EXTENSION);

    let bytes = script.code.as_bytes();
    let file = File::create(&path).await?;
    let mut buffer = BufWriter::new(file);
    buffer.write_all(bytes).await?;
    buffer.flush().await?;
    Ok(())
  } else {
    Err(Error::from(NotADirectory))
  }
}

#[tauri::command]
pub async fn get_script(app: AppHandle, id: ScriptId) -> Result<Script> {
  app
    .client(async |cl| cl.get_script(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_scripts(app: AppHandle) -> Result<Vec<Script>> {
  app
    .client(async |cl| cl.get_scripts().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn import_scripts(app: AppHandle, paths: Vec<PathBuf>) -> Result<Vec<ScriptId>> {
  if !paths.is_empty() {
    let owner = app.nil().player().await?;
    let mut scripts = Vec::with_capacity(paths.len());

    for (i, path) in paths.iter().enumerate() {
      if path.is_file() {
        let code = fs::read_to_string(path).await?;
        let name = path
          .file_stem()
          .and_then(OsStr::to_str)
          .map(ToOwned::to_owned)
          .unwrap_or_else(|| format!("Script {i}"));

        scripts.push(Script {
          id: ScriptId::default(),
          name,
          code,
          owner: owner.clone(),
        });
      }
    }

    if !scripts.is_empty() {
      return add_scripts(app, scripts).await;
    }
  }

  Ok(Vec::new())
}

#[tauri::command]
pub async fn remove_script(app: AppHandle, id: ScriptId) -> Result<()> {
  app
    .client(async |cl| cl.remove_script(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn update_script(app: AppHandle, script: Script) -> Result<()> {
  app
    .client(async |cl| cl.update_script(script).await)
    .await?
    .map_err(Into::into)
}
