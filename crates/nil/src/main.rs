// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![feature(iterator_try_collect, try_blocks)]

mod command;
mod error;
mod manager;
mod plugin;
mod state;
mod window;

#[cfg(debug_assertions)]
mod log;

use error::BoxResult;
use state::Nil;
use tauri::{AppHandle, Manager};

fn main() {
  #[cfg(debug_assertions)]
  log::setup();

  let mut builder = tauri::Builder::default();

  if cfg!(not(debug_assertions)) {
    builder = builder.plugin(plugin::single_instance());
  }

  builder
    .plugin(plugin::on_exit())
    .plugin(plugin::prevent_default())
    .plugin(plugin::window_state())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_persisted_scope::init())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![
      command::is_dev,
      command::is_host,
      command::show_window,
      command::chat::get_chat_messages,
      command::chat::push_chat_message,
      command::cheat::cheat_set_building_level,
      command::cheat::cheat_set_food,
      command::cheat::cheat_set_iron,
      command::cheat::cheat_set_max_infrastructure,
      command::cheat::cheat_set_max_resources,
      command::cheat::cheat_set_resources,
      command::cheat::cheat_set_stability,
      command::cheat::cheat_set_stone,
      command::cheat::cheat_set_wood,
      command::client::start_client,
      command::client::stop_client,
      command::continent::get_field,
      command::continent::get_fields,
      command::continent::get_continent_size,
      command::infrastructure::prefecture::add_prefecture_build_order,
      command::infrastructure::prefecture::cancel_prefecture_build_order,
      command::infrastructure::prefecture::get_prefecture_build_catalog,
      command::lobby::get_lobby,
      command::lobby::set_lobby_ready,
      command::player::get_player,
      command::player::get_player_coords,
      command::player::get_players,
      command::player::player_exists,
      command::player::set_player_status,
      command::player::spawn_player,
      command::player::spawn_player_village,
      command::round::end_turn,
      command::round::get_round,
      command::round::is_round_idle,
      command::round::start_round,
      command::script::add_script,
      command::script::add_scripts,
      command::script::export_script,
      command::script::get_script,
      command::script::get_scripts,
      command::script::import_scripts,
      command::script::remove_script,
      command::script::update_script,
      command::server::get_server_addr,
      command::server::get_server_version,
      command::server::is_server_ready,
      command::server::start_server_with_options,
      command::server::start_server_with_savedata,
      command::server::stop_server,
      command::village::get_village,
      command::world::get_world_config,
      command::world::get_world_stats,
      command::world::save_world,
    ])
    .run(tauri::generate_context!())
    .expect("failed to start nil");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  app.manage(Nil::new(app));
  window::open(app)?;
  Ok(())
}
