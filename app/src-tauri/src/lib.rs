// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(iterator_try_collect, try_blocks)]

mod command;
mod error;
mod event;
mod manager;
mod plugin;
mod state;
mod window;

#[cfg(desktop)]
mod tray;

use error::BoxResult;
use state::Nil;
use tauri::{AppHandle, Manager, Wry};

#[expect(clippy::too_many_lines)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  builder()
    .plugin(plugin::on_exit())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_persisted_scope::init())
    .plugin(tauri_plugin_process::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![
      command::allow_scope,
      command::args,
      command::create_tray_icon,
      command::current_dir,
      command::current_exe,
      command::exists,
      command::home_dir,
      command::is_host,
      command::is_local,
      command::is_local_and_host,
      command::is_remote,
      command::is_remote_or_host,
      command::nil_dir,
      command::now,
      command::open_devtools,
      command::show_window,
      command::version,
      command::battle::simulate_battle,
      command::chat::get_chat_history,
      command::chat::push_chat_message,
      command::cheat::behavior::cheat_get_build_steps,
      command::cheat::city::cheat_set_stability,
      command::cheat::infrastructure::cheat_get_academy_recruit_queue,
      command::cheat::infrastructure::cheat_get_academy_recruit_queues,
      command::cheat::infrastructure::cheat_get_all_academy_recruit_queues,
      command::cheat::infrastructure::cheat_get_all_prefecture_build_queues,
      command::cheat::infrastructure::cheat_get_all_stable_recruit_queues,
      command::cheat::infrastructure::cheat_get_infrastructure,
      command::cheat::infrastructure::cheat_get_prefecture_build_queue,
      command::cheat::infrastructure::cheat_get_prefecture_build_queues,
      command::cheat::infrastructure::cheat_get_stable_recruit_queue,
      command::cheat::infrastructure::cheat_get_stable_recruit_queues,
      command::cheat::infrastructure::cheat_get_storage_capacity,
      command::cheat::infrastructure::cheat_set_building_level,
      command::cheat::infrastructure::cheat_set_max_infrastructure,
      command::cheat::military::cheat_get_idle_armies_at,
      command::cheat::military::cheat_get_idle_personnel_at,
      command::cheat::military::cheat_spawn_personnel,
      command::cheat::npc::cheat_get_ethics,
      command::cheat::npc::cheat_set_bot_ethics,
      command::cheat::npc::cheat_spawn_bot,
      command::cheat::player::cheat_get_player,
      command::cheat::player::cheat_get_players,
      command::cheat::resources::cheat_get_resources,
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
      command::cheat::round::cheat_skip_round,
      command::city::get_city,
      command::city::get_city_score,
      command::city::get_public_cities,
      command::city::get_public_city,
      command::city::rename_city,
      command::city::search_city,
      command::city::search_public_city,
      command::client::get_client_version,
      command::client::stop_client,
      command::client::update_client,
      command::continent::get_bulk_distance,
      command::continent::get_continent_size,
      command::continent::get_distance,
      command::continent::get_public_field,
      command::continent::get_public_fields,
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
      command::infrastructure::workshop::add_workshop_recruit_order,
      command::infrastructure::workshop::cancel_workshop_recruit_order,
      command::infrastructure::workshop::get_workshop_recruit_catalog,
      command::military::request_maneuver,
      command::npc::bot::get_bot_coords,
      command::npc::bot::get_public_bot,
      command::npc::bot::get_public_bots,
      command::npc::precursor::get_precursor_coords,
      command::npc::precursor::get_public_precursor,
      command::npc::precursor::get_public_precursors,
      command::player::get_player,
      command::player::get_player_coords,
      command::player::get_player_maintenance,
      command::player::get_player_military,
      command::player::get_player_reports,
      command::player::get_player_status,
      command::player::get_player_storage_capacity,
      command::player::get_player_worlds,
      command::player::get_public_player,
      command::player::get_public_players,
      command::player::player_exists,
      command::player::set_player_status,
      command::player::spawn_player,
      command::ranking::get_rank,
      command::ranking::get_ranking,
      command::report::get_report,
      command::report::get_reports,
      command::round::get_round,
      command::round::is_round_idle,
      command::round::is_round_waiting,
      command::round::set_player_ready,
      command::round::start_round,
      command::script::execute_script,
      command::script::execute_script_at,
      command::script::import_script,
      command::script::import_scripts,
      command::script::is_script,
      command::script::script_dir,
      command::server::authorize,
      command::server::get_server_addr,
      command::server::get_server_kind,
      command::server::get_server_version,
      command::server::is_server_ready,
      command::server::start_server_with_options,
      command::server::start_server_with_savedata,
      command::server::stop_server,
      command::server::validate_token,
      command::user::create_user,
      command::user::user_exists,
      command::world::create_remote_world,
      command::world::get_remote_world,
      command::world::get_remote_worlds,
      command::world::get_savedata_players,
      command::world::get_world_bots,
      command::world::get_world_config,
      command::world::get_world_players,
      command::world::get_world_precursors,
      command::world::get_world_stats,
      command::world::is_savedata,
      command::world::read_savedata_info,
      command::world::save_local_world,
      command::world::savedata_dir,
    ])
    .run(tauri::generate_context!())
    .expect("Failed to start Call of Nil app");
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
  app.plugin(plugin::pinia(app)?)?;
  app.manage(Nil::new(app)?);

  #[cfg(desktop)]
  window::desktop::open(app)?;
  #[cfg(mobile)]
  window::mobile::open(app)?;

  Ok(())
}
