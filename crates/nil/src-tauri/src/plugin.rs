use crate::manager::WindowExt;
use anyhow::Result;
use tauri::Wry;
use tauri::plugin::TauriPlugin;

pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::{Builder, Flags};

  #[cfg_attr(not(windows), allow(unused_mut))]
  let mut builder = Builder::new().with_flags(Flags::debug());

  #[cfg(windows)]
  {
    use tauri_plugin_prevent_default::WindowsOptions;
    builder = builder.platform(WindowsOptions {
      general_autofill: false,
      password_autosave: false,
    });
  };

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
    .with_state_flags(Flags::POSITION)
    .build()
}
