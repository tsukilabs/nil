// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(iterator_try_collect, try_blocks)]

mod command;
mod error;
mod manager;
mod plugin;
mod state;
mod window;

#[cfg(debug_assertions)]
mod log;
#[cfg(desktop)]
mod tray;

use error::BoxResult;
use state::Nil;
use tauri::{AppHandle, Manager, Wry};

#[allow(clippy::too_many_lines)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(debug_assertions)]
  log::setup();

  builder()
    .plugin(plugin::on_exit())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_persisted_scope::init())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![
      command::create_tray_icon,
      command::is_host,
      command::show_window,
      command::version,
      command::chat::get_chat_history,
      command::chat::push_chat_message,
      command::cheat::city::cheat_set_stability,
      command::cheat::infrastructure::cheat_set_building_level,
      command::cheat::infrastructure::cheat_set_max_infrastructure,
      command::cheat::npc::cheat_get_bot_resources,
      command::cheat::npc::cheat_get_bot_storage_capacity,
      command::cheat::npc::cheat_get_precursor_resources,
      command::cheat::npc::cheat_get_precursor_storage_capacity,
      command::cheat::npc::cheat_spawn_bot,
      command::cheat::resources::cheat_set_food,
      command::cheat::resources::cheat_set_iron,
      command::cheat::resources::cheat_set_max_food,
      command::cheat::resources::cheat_set_max_iron,
      command::cheat::resources::cheat_set_max_resources,
      command::cheat::resources::cheat_set_max_silo_resources,
      command::cheat::resources::cheat_set_max_stone,
      command::cheat::resources::cheat_set_max_warehouse_resources,
      command::cheat::resources::cheat_set_max_wood,
      command::cheat::resources::cheat_set_resources,
      command::cheat::resources::cheat_set_stone,
      command::cheat::resources::cheat_set_wood,
      command::city::get_city,
      command::city::get_city_score,
      command::city::get_public_city,
      command::city::rename_city,
      command::client::start_client,
      command::client::stop_client,
      command::continent::get_field,
      command::continent::get_fields,
      command::continent::get_continent_size,
      command::infrastructure::toggle_building,
      command::infrastructure::academy::add_academy_recruit_order,
      command::infrastructure::academy::cancel_academy_recruit_order,
      command::infrastructure::academy::get_academy_recruit_catalog,
      command::infrastructure::prefecture::add_prefecture_build_order,
      command::infrastructure::prefecture::cancel_prefecture_build_order,
      command::infrastructure::prefecture::get_prefecture_build_catalog,
      command::infrastructure::stable::add_stable_recruit_order,
      command::infrastructure::stable::cancel_stable_recruit_order,
      command::infrastructure::stable::get_stable_recruit_catalog,
      command::npc::bot::get_bot_coords,
      command::npc::bot::get_public_bot,
      command::npc::precursor::get_precursor_coords,
      command::npc::precursor::get_public_precursor,
      command::player::get_player,
      command::player::get_player_coords,
      command::player::get_player_maintenance,
      command::player::get_player_military,
      command::player::get_player_status,
      command::player::get_player_storage_capacity,
      command::player::get_players,
      command::player::get_public_player,
      command::player::get_public_players,
      command::player::player_exists,
      command::player::set_player_status,
      command::player::spawn_player,
      command::ranking::get_rank,
      command::ranking::get_ranking,
      command::round::end_turn,
      command::round::get_round,
      command::round::is_round_idle,
      command::round::start_round,
      command::server::get_server_addr,
      command::server::get_server_version,
      command::server::is_server_ready,
      command::server::start_server_with_options,
      command::server::start_server_with_savedata,
      command::server::stop_server,
      command::world::get_world_config,
      command::world::get_world_stats,
      command::world::save_world,
    ])
    .run(tauri::generate_context!())
    .expect("Failed to start nil");
}

#[cfg(desktop)]
fn builder() -> tauri::Builder<Wry> {
  let mut builder = tauri::Builder::default()
    .plugin(plugin::prevent_default())
    .plugin(tauri_plugin_updater::Builder::new().build());

  if !cfg!(debug_assertions) {
    builder = builder.plugin(plugin::single_instance());
  }

  builder
}

#[cfg(mobile)]
fn builder() -> tauri::Builder<Wry> {
  tauri::Builder::default()
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  let app_dir = app.path().app_data_dir()?;
  let pinia = tauri_plugin_pinia::Builder::new()
    .path(app_dir.join("settings"))
    .build();

  app.plugin(pinia)?;
  app.manage(Nil::new(app));

  #[cfg(desktop)]
  window::desktop::open(app)?;
  #[cfg(mobile)]
  window::mobile::open(app)?;

  Ok(())
}
