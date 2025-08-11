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
  let builder = builder.platform(PlatformOptions {
    general_autofill: false,
    password_autosave: false,
  });

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

pub fn window_state() -> TauriPlugin<Wry> {
  use tauri_plugin_window_state::StateFlags as Flags;

  tauri_plugin_window_state::Builder::new()
    .with_state_flags(Flags::MAXIMIZED | Flags::POSITION)
    .build()
}
