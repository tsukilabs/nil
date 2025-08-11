// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder, Wry};

pub trait WindowExt {
  fn main_window(&self) -> WebviewWindow<Wry>;
}

impl<T: Manager<Wry>> WindowExt for T {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

pub fn open(app: &AppHandle) -> Result<()> {
  WebviewWindowBuilder::new(app, "main", super::url())
    .title("Call of Nil")
    .initialization_script(super::script())
    .inner_size(1280.0, 768.0)
    .min_inner_size(800.0, 600.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .prevent_overflow()
    .build()?;

  Ok(())
}
