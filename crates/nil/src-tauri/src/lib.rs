#![feature(iterator_try_collect, let_chains, result_flattening, try_blocks)]

mod command;
mod error;
mod manager;
mod state;

#[cfg(desktop)]
mod plugin;

#[cfg(feature = "tracing")]
mod log;

use anyhow::Result;
use error::BoxResult;
use state::Nil;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(desktop)]
  let builder = {
    tauri::Builder::default()
      .plugin(plugin::single_instance())
      .plugin(plugin::prevent_default())
      .plugin(plugin::window_state())
  };

  #[cfg(mobile)]
  let builder = tauri::Builder::default();

  builder
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![
      command::is_dev,
      command::show_window,
      command::client::start_client,
      command::client::stop_client,
      command::player::get_player,
      command::player::get_player_villages,
      command::player::spawn_player,
      command::server::get_server_version,
      command::server::is_server_ready,
      command::server::start_server,
      command::server::stop_server,
      command::village::get_village
    ])
    .run(tauri::generate_context!())
    .expect("failed to start nil");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  #[cfg(feature = "tracing")]
  log::setup()?;

  app.manage(Nil::new(app));

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
