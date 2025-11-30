// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder, WindowEvent, Wry};
use tauri_plugin_pinia::ManagerExt as _;

pub trait WindowExt {
  fn main_window(&self) -> WebviewWindow<Wry>;
}

impl<T: Manager<Wry>> WindowExt for T {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

pub fn open(app: &AppHandle) -> Result<()> {
  let window = WebviewWindowBuilder::new(app, "main", super::url())
    .title("Call of Nil")
    .initialization_script(super::script())
    .inner_size(1280.0, 768.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .prevent_overflow()
    .build()?;

  window.on_window_event(on_window_event(app));

  Ok(())
}

fn on_window_event(app: &AppHandle) -> impl Fn(&WindowEvent) + use<> {
  let app = app.clone();
  move |event| {
    if let WindowEvent::CloseRequested { api, .. } = event
      && app
        .pinia()
        .get_or_default("settings", "hideOnClose")
    {
      api.prevent_close();
      let _ = app.main_window().hide();
    }
  }
}
