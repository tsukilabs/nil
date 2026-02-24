// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::manager::ManagerExt;
use futures::TryFutureExt;
use natord::compare_ignore_case;
use nil_lua::script::{Script, ScriptOutput};
use std::path::{Path, PathBuf};
use tauri::AppHandle;
use tokio::fs;

#[tauri::command]
pub async fn execute_script(app: AppHandle, chunk: String) -> Result<ScriptOutput> {
  app
    .lua(async |lua| lua.execute(&chunk).await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn execute_script_at(app: AppHandle, path: PathBuf) -> Result<ScriptOutput> {
  fs::read_to_string(path)
    .map_err(Into::<Error>::into)
    .and_then(|chunk| execute_script(app, chunk))
    .await
}

#[tauri::command]
pub async fn import_script(app: AppHandle, path: PathBuf) -> Result<()> {
  import_scripts(app, vec![path]).await
}

#[tauri::command]
pub async fn import_scripts(app: AppHandle, paths: Vec<PathBuf>) -> Result<()> {
  let dir = app.script_dir()?;
  fs::create_dir_all(&dir).await?;

  for path in paths {
    if is_script(&path).await?
      && let Some(name) = path.file_stem()
      && let Some(name) = name.to_str()
    {
      let mut new = dir.join(name);
      let mut suffix = 1;

      'inner: loop {
        if suffix > 1 {
          new.set_file_name(format!("{name}_{suffix}.lua"));
        } else {
          new.set_file_name(format!("{name}.lua"));
        }

        if fs::try_exists(&new).await? {
          suffix += 1;
        } else {
          break 'inner;
        }
      }

      fs::copy(path, new).await?;
    }
  }

  Ok(())
}

#[tauri::command]
pub async fn is_script(path: &Path) -> Result<bool> {
  if let Some(ext) = path.extension()
    && ext.eq_ignore_ascii_case("lua")
  {
    let metadata = fs::metadata(&path).await?;
    Ok(metadata.is_file() && metadata.len() > 0)
  } else {
    Ok(false)
  }
}

#[tauri::command]
pub async fn load_scripts(app: AppHandle) -> Result<Vec<Script>> {
  let mut scripts = Vec::new();
  let dir = app.script_dir()?;
  let mut entries = fs::read_dir(dir).await?;

  while let Some(entry) = entries.next_entry().await? {
    let path = entry.path();
    if is_script(&path).await? {
      scripts.push(Script::load(path).await?);
    }
  }

  scripts.sort_by(|a, b| compare_ignore_case(&a.name, &b.name));

  Ok(scripts)
}

#[tauri::command]
pub async fn script_dir(app: AppHandle) -> Result<PathBuf> {
  app.script_dir()
}
