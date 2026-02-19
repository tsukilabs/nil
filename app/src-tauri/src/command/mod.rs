// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![expect(clippy::needless_pass_by_value, clippy::wildcard_imports)]

pub mod battle;
pub mod chat;
pub mod cheat;
pub mod city;
pub mod client;
pub mod continent;
pub mod infrastructure;
pub mod military;
pub mod npc;
pub mod player;
pub mod ranking;
pub mod report;
pub mod round;
pub mod script;
pub mod server;
pub mod user;
pub mod world;

use crate::error::{Error, Result};
use crate::manager::ManagerExt;
use jiff::Zoned;
use std::env;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::async_runtime::spawn_blocking;
use tauri_plugin_fs::FsExt;

#[cfg(desktop)]
use {crate::tray, tauri::WebviewWindow};

#[tauri::command]
pub async fn allow_scope(app: AppHandle, path: PathBuf) -> Result<()> {
  let task = spawn_blocking(move || {
    let scope = app.fs_scope();
    if !scope.is_allowed(&path) {
      if path.is_file() {
        scope.allow_file(path)?;
      } else if path.is_dir() {
        scope.allow_directory(path, true)?;
      }
    }

    Ok::<_, Error>(())
  });

  task.await??;

  Ok(())
}

#[tauri::command]
pub fn args() -> Vec<String> {
  env::args().collect()
}

#[cfg(desktop)]
#[tauri::command]
pub fn create_tray_icon(app: AppHandle) -> Result<()> {
  tray::create(&app)?;
  Ok(())
}

#[cfg(mobile)]
#[tauri::command]
pub fn create_tray_icon() {}

#[tauri::command]
pub fn current_dir() -> Result<PathBuf> {
  Ok(env::current_dir()?)
}

#[tauri::command]
pub fn current_exe() -> Result<PathBuf> {
  Ok(env::current_exe()?)
}

#[tauri::command]
pub async fn exists(path: PathBuf) -> Result<bool> {
  Ok(tokio::fs::try_exists(path).await?)
}

#[tauri::command]
pub fn home_dir() -> Option<PathBuf> {
  env::home_dir()
}

#[tauri::command]
pub async fn is_host(app: AppHandle) -> bool {
  app.nil().is_host().await
}

#[tauri::command]
pub async fn is_local(app: AppHandle) -> bool {
  app.nil().is_local().await
}

#[tauri::command]
pub async fn is_local_and_host(app: AppHandle) -> bool {
  app.nil().is_local_and_host().await
}

#[tauri::command]
pub async fn is_remote(app: AppHandle) -> bool {
  app.nil().is_remote().await
}

#[tauri::command]
pub async fn is_remote_or_host(app: AppHandle) -> bool {
  app.nil().is_remote_or_host().await
}

#[tauri::command]
pub async fn nil_dir(app: AppHandle) -> Result<PathBuf> {
  app.nil_dir()
}

#[tauri::command]
pub fn now() -> Zoned {
  Zoned::now()
}

#[cfg(desktop)]
#[tauri::command]
pub fn show_window(window: WebviewWindow) -> Result<()> {
  window.show()?;
  window.unminimize()?;
  window.set_focus()?;
  Ok(())
}

#[cfg(mobile)]
#[tauri::command]
pub fn show_window() {}

#[tauri::command]
pub fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}
