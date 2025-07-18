// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod chat;
pub mod cheat;
pub mod client;
pub mod continent;
pub mod infrastructure;
pub mod nsr;
pub mod player;
pub mod round;
pub mod script;
pub mod server;
pub mod village;
pub mod world;

use crate::error::Result;
use crate::manager::ManagerExt;
use tauri::{AppHandle, WebviewWindow};

#[tauri::command]
pub async fn is_host(app: AppHandle) -> bool {
  app.nil().is_host().await
}

#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> Result<()> {
  window
    .show()
    .and_then(|()| window.unminimize())
    .and_then(|()| window.set_focus())
    .map_err(Into::into)
}
