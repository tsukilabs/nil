use tauri::Wry;
use tauri::plugin::TauriPlugin;

pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::Flags;

  #[cfg_attr(not(windows), allow(unused_mut))]
  let mut builder = tauri_plugin_prevent_default::Builder::new()
    .with_flags(Flags::all().difference(Flags::DEV_TOOLS));

  #[cfg(windows)]
  {
    builder = builder
      .general_autofill(false)
      .password_autosave(false)
  };

  builder.build()
}

pub fn window_state() -> TauriPlugin<Wry> {
  use tauri_plugin_window_state::StateFlags as Flags;

  tauri_plugin_window_state::Builder::new()
    .with_state_flags(Flags::POSITION)
    .build()
}
