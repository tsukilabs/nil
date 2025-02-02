#![feature(iterator_try_collect, let_chains, result_flattening, try_blocks)]

mod command;
mod error;
mod manager;

#[cfg(desktop)]
mod plugin;

#[cfg(feature = "tracing")]
mod log;

use anyhow::Result;
use error::BoxResult;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(desktop)]
  let builder = {
    tauri::Builder::default()
      .plugin(plugin::prevent_default())
      .plugin(plugin::window_state())
  };

  #[cfg(mobile)]
  let builder = tauri::Builder::default();

  builder
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![command::show_window])
    .run(tauri::generate_context!())
    .expect("failed to start nil");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  #[cfg(feature = "tracing")]
  log::setup()?;

  open_window(app)?;
  Ok(())
}

fn open_window(app: &AppHandle) -> Result<()> {
  let url = WebviewUrl::App("index.html".into());

  #[cfg(desktop)]
  WebviewWindowBuilder::new(app, "main", url)
    .title("Nil")
    .inner_size(800.0, 600.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .build()?;

  #[cfg(mobile)]
  WebviewWindowBuilder::new(app, "main", url).build()?;

  Ok(())
}
