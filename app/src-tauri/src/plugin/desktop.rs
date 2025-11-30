// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::window::desktop::WindowExt;
use anyhow::Result;
use tauri::Wry;
use tauri::plugin::TauriPlugin;

pub fn prevent_default() -> TauriPlugin<Wry> {
  #[cfg(windows)]
  use tauri_plugin_prevent_default::PlatformOptions;
  use tauri_plugin_prevent_default::{Builder, Flags};

  let builder = Builder::new().with_flags(Flags::debug());

  #[cfg(windows)]
  let builder = builder.platform(
    PlatformOptions::new()
      .browser_accelerator_keys(cfg!(debug_assertions))
      .default_context_menus(cfg!(debug_assertions))
      .default_script_dialogs(cfg!(debug_assertions))
      .general_autofill(false)
      .password_autosave(false)
      .pinch_zoom(false)
      .zoom_control(false),
  );

  builder.build()
}

pub fn single_instance() -> TauriPlugin<Wry> {
  tauri_plugin_single_instance::init(|app, _, _| {
    let window = app.main_window();
    let _: Result<()> = try {
      window.show()?;
      window.unminimize()?;
      window.set_focus()?;
    };
  })
}
