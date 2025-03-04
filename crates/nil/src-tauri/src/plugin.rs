use crate::manager::{ManagerExt, WindowExt};
use anyhow::Result;
use tauri::async_runtime::block_on;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{RunEvent, Wry};

pub fn on_exit() -> TauriPlugin<Wry> {
  Builder::new("on-exit")
    .on_event(|app, event| {
      if let RunEvent::Exit = event {
        block_on(async {
          let nil = app.nil();
          let _ = nil.stop_client().await;
          nil.stop_server().await;
        });
      }
    })
    .build()
}

pub fn prevent_default() -> TauriPlugin<Wry> {
  #[cfg(windows)]
  use tauri_plugin_prevent_default::WindowsOptions;
  use tauri_plugin_prevent_default::{Builder, Flags};

  let builder = Builder::new().with_flags(Flags::debug());

  #[cfg(windows)]
  let builder = builder.platform(WindowsOptions {
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
    .with_state_flags(Flags::POSITION)
    .build()
}
