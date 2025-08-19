// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![allow(clippy::needless_pass_by_value)]

pub mod chat;
pub mod cheat;
pub mod city;
pub mod client;
pub mod continent;
pub mod infrastructure;
pub mod npc;
pub mod nsr;
pub mod player;
pub mod ranking;
pub mod round;
pub mod script;
pub mod server;
pub mod world;

use crate::manager::ManagerExt;
use tauri::AppHandle;

#[cfg(desktop)]
use {crate::error::Result, crate::tray, tauri::WebviewWindow};

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
pub async fn is_host(app: AppHandle) -> bool {
  app.nil().is_host().await
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
