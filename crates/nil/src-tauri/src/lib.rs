#![feature(iterator_try_collect, let_chains, result_flattening, try_blocks)]

mod command;
mod error;
mod manager;
mod state;

#[cfg(desktop)]
mod plugin;

use anyhow::Result;
use error::BoxResult;
use state::Nil;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(desktop)]
  let builder = {
    let mut builder = tauri::Builder::default()
      .plugin(plugin::prevent_default())
      .plugin(plugin::window_state());

    if !cfg!(debug_assertions) {
      builder = builder.plugin(plugin::single_instance());
    }

    builder
  };

  #[cfg(mobile)]
  let builder = tauri::Builder::default();

  builder
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![
      command::get_server_addr,
      command::get_server_version,
      command::is_dev,
      command::is_host,
      command::is_mobile,
      command::is_server_ready,
      command::show_window,
      command::start_client,
      command::start_server,
      command::stop_client,
      command::stop_server,
      command::player::get_player,
      command::player::get_player_villages,
      command::player::get_players,
      command::player::spawn_player,
      command::round::get_round_state,
      command::round::is_round_idle,
      command::village::get_village,
      command::world::get_world_state,
    ])
    .run(tauri::generate_context!())
    .expect("failed to start nil");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  app.manage(Nil::new(app));
  open_window(app)?;
  Ok(())
}

fn open_window(app: &AppHandle) -> Result<()> {
  let url = WebviewUrl::App("index.html".into());

  #[cfg(desktop)]
  let builder = WebviewWindowBuilder::new(app, "main", url)
    .title("Nil")
    .inner_size(800.0, 600.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false);

  #[cfg(mobile)]
  let builder = WebviewWindowBuilder::new(app, "main", url);

  builder.build()?;

  Ok(())
}
