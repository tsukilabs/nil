// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(nonpoison_mutex, sync_nonpoison)]
#![expect(clippy::missing_safety_doc)]

mod client;
mod json;
mod macros;
mod queue;
mod request;
mod response;
mod status;

use crate::request::next_request_id;
use client::CLIENT;
use futures::future::BoxFuture;
use json::{deserialize_ptr, serialize};
use nil_core::event::Event;
use std::ffi::{CString, c_char};
use std::ptr;
use std::sync::LazyLock;
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};

pub use client::UpdateClient;
pub use request::RequestId;
pub use response::{Response, Result};
pub use status::Status;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
  RuntimeBuilder::new_multi_thread()
    .enable_all()
    .thread_name("callofnil-tokio")
    .build()
    .expect("failed to initialize tokio runtime")
});

///////////////////////////////
//////////// FFI //////////////
///////////////////////////////

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_ffi_free_str(ptr: *mut c_char) {
  if !ptr.is_null() {
    drop(unsafe { CString::from_raw(ptr) });
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_ffi_poll(out: *mut *mut c_char) -> Status {
  if out.is_null() {
    return Status::ERR_NULL_POINTER;
  }

  unsafe { *out = ptr::null_mut() };

  match queue::poll() {
    Some(entry) => {
      match serialize(&entry) {
        Ok(json) => {
          let json = CString::new(json).unwrap();
          unsafe { *out = json.into_raw() };
          Status::OK
        }
        Err(_) => Status::ERR_SERIALIZATION,
      }
    }
    None => Status::ERR_NOTHING_TO_POLL,
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_ffi_shutdown() {
  queue::clear();
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_ffi_version() -> RequestId {
  push_ok!(env!("CARGO_PKG_VERSION"))
}

///////////////////////////////
//////////// CLIENT ///////////
///////////////////////////////

#[unsafe(no_mangle)]
pub extern "C" fn nil_client_version() -> RequestId {
  push_ok!(nil_client::VERSION)
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_is_local() -> RequestId {
  async_push_ok!(CLIENT.read().await.is_local())
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_is_ready() -> RequestId {
  async_push_ok!(CLIENT.read().await.is_ready().await)
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_is_remote() -> RequestId {
  async_push_ok!(CLIENT.read().await.is_remote())
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_server_addr() -> RequestId {
  async_push_ok!(CLIENT.read().await.server_addr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_set_user_agent(user_agent: *const c_char) -> RequestId {
  let id = next_request_id();
  if user_agent.is_null() {
    queue::push_err(id, Status::ERR_NULL_POINTER);
  } else {
    match unsafe { deserialize_ptr::<String>(user_agent) } {
      Ok(user_agent) => {
        RUNTIME.spawn(async move {
          CLIENT
            .write()
            .await
            .set_user_agent(&user_agent);

          queue::push_ok(id, ());
        });
      }
      Err(err) => {
        queue::push_err(id, err);
      }
    }
  }

  id
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_stop_client() -> RequestId {
  let id = next_request_id();
  RUNTIME.spawn(async move {
    CLIENT.write().await.stop().await;
  });

  id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_update_client(options: *const c_char) -> RequestId {
  let id = next_request_id();
  if options.is_null() {
    queue::push_err(id, Status::ERR_NULL_POINTER);
  } else {
    type OnEvent = fn(Event) -> BoxFuture<'static, ()>;
    match unsafe { deserialize_ptr::<UpdateClient>(options) } {
      Ok(options) => {
        RUNTIME.spawn(async move {
          let result = CLIENT
            .write()
            .await
            .update::<OnEvent>(options.server)
            .maybe_world_id(options.world_id)
            .maybe_world_password(options.world_password)
            .maybe_player_id(options.player_id)
            .maybe_player_password(options.player_password)
            .maybe_authorization_token(options.authorization_token)
            .call()
            .await;

          queue::push_result(id, Result::from(result));
        });
      }
      Err(err) => {
        queue::push_err(id, err);
      }
    }
  }

  id
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_user_agent() -> RequestId {
  async_push_ok!(CLIENT.read().await.user_agent().to_owned())
}

#[unsafe(no_mangle)]
pub extern "C" fn nil_world() -> RequestId {
  let id = next_request_id();
  RUNTIME.spawn(async move {
    match CLIENT.read().await.world() {
      Some(world) => queue::push_ok(id, world),
      None => queue::push_ok(id, None::<&str>),
    }
  });

  id
}

///////////////////////////////
/////////// SERVER ////////////
///////////////////////////////

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_academy_recruit_order(req: *const c_char) -> RequestId {
  send!(add_academy_recruit_order, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_prefecture_build_order(req: *const c_char) -> RequestId {
  send!(add_prefecture_build_order, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_stable_recruit_order(req: *const c_char) -> RequestId {
  send!(add_stable_recruit_order, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_workshop_recruit_order(req: *const c_char) -> RequestId {
  send!(add_workshop_recruit_order, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_authorize(req: *const c_char) -> RequestId {
  send!(authorize, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_academy_recruit_order(req: *const c_char) -> RequestId {
  send!(cancel_academy_recruit_order, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_maneuver(req: *const c_char) -> RequestId {
  send!(cancel_maneuver, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_prefecture_build_order(req: *const c_char) -> RequestId {
  send!(cancel_prefecture_build_order, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_stable_recruit_order(req: *const c_char) -> RequestId {
  send!(cancel_stable_recruit_order, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_workshop_recruit_order(req: *const c_char) -> RequestId {
  send!(cancel_workshop_recruit_order, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_fill_world(req: *const c_char) -> RequestId {
  send!(cheat_fill_world, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_academy_recruit_queue(req: *const c_char) -> RequestId {
  send!(cheat_get_academy_recruit_queue, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_academy_recruit_queues(req: *const c_char) -> RequestId {
  send!(cheat_get_academy_recruit_queues, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_academy_recruit_queues(req: *const c_char) -> RequestId {
  send!(cheat_get_all_academy_recruit_queues, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_prefecture_build_queues(
  req: *const c_char,
) -> RequestId {
  send!(cheat_get_all_prefecture_build_queues, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_stable_recruit_queues(req: *const c_char) -> RequestId {
  send!(cheat_get_all_stable_recruit_queues, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_build_steps(req: *const c_char) -> RequestId {
  send!(cheat_get_build_steps, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_cities(req: *const c_char) -> RequestId {
  send!(cheat_get_cities, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_city(req: *const c_char) -> RequestId {
  send!(cheat_get_city, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_ethics(req: *const c_char) -> RequestId {
  send!(cheat_get_ethics, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_idle_armies_at(req: *const c_char) -> RequestId {
  send!(cheat_get_idle_armies_at, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_idle_personnel_at(req: *const c_char) -> RequestId {
  send!(cheat_get_idle_personnel_at, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_infrastructure(req: *const c_char) -> RequestId {
  send!(cheat_get_infrastructure, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_maneuvers(req: *const c_char) -> RequestId {
  send!(cheat_get_maneuvers, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_maneuvers_of(req: *const c_char) -> RequestId {
  send!(cheat_get_maneuvers_of, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_player(req: *const c_char) -> RequestId {
  send!(cheat_get_player, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_players(req: *const c_char) -> RequestId {
  send!(cheat_get_players, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_prefecture_build_queue(req: *const c_char) -> RequestId {
  send!(cheat_get_prefecture_build_queue, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_prefecture_build_queues(req: *const c_char) -> RequestId {
  send!(cheat_get_prefecture_build_queues, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_resources(req: *const c_char) -> RequestId {
  send!(cheat_get_resources, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_stable_recruit_queue(req: *const c_char) -> RequestId {
  send!(cheat_get_stable_recruit_queue, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_stable_recruit_queues(req: *const c_char) -> RequestId {
  send!(cheat_get_stable_recruit_queues, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_storage_capacity(req: *const c_char) -> RequestId {
  send!(cheat_get_storage_capacity, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_bot_ethics(req: *const c_char) -> RequestId {
  send!(cheat_set_bot_ethics, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_building_level(req: *const c_char) -> RequestId {
  send!(cheat_set_building_level, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_food(req: *const c_char) -> RequestId {
  send!(cheat_set_food, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_iron(req: *const c_char) -> RequestId {
  send!(cheat_set_iron, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_food(req: *const c_char) -> RequestId {
  send!(cheat_set_max_food, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_infrastructure(req: *const c_char) -> RequestId {
  send!(cheat_set_max_infrastructure, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_iron(req: *const c_char) -> RequestId {
  send!(cheat_set_max_iron, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_resources(req: *const c_char) -> RequestId {
  send!(cheat_set_max_resources, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_silo_resources(req: *const c_char) -> RequestId {
  send!(cheat_set_max_silo_resources, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_stone(req: *const c_char) -> RequestId {
  send!(cheat_set_max_stone, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_warehouse_resources(req: *const c_char) -> RequestId {
  send!(cheat_set_max_warehouse_resources, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_wood(req: *const c_char) -> RequestId {
  send!(cheat_set_max_wood, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_resources(req: *const c_char) -> RequestId {
  send!(cheat_set_resources, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_stability(req: *const c_char) -> RequestId {
  send!(cheat_set_stability, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_stone(req: *const c_char) -> RequestId {
  send!(cheat_set_stone, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_wood(req: *const c_char) -> RequestId {
  send!(cheat_set_wood, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_skip_round(req: *const c_char) -> RequestId {
  send!(cheat_skip_round, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_bot(req: *const c_char) -> RequestId {
  send!(cheat_spawn_bot, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_city(req: *const c_char) -> RequestId {
  send!(cheat_spawn_city, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_personnel(req: *const c_char) -> RequestId {
  send!(cheat_spawn_personnel, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_create_remote_world(req: *const c_char) -> RequestId {
  send!(create_remote_world, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_create_user(req: *const c_char) -> RequestId {
  send!(create_user, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_delete_remote_world(req: *const c_char) -> RequestId {
  send!(delete_remote_world, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_forward_report(req: *const c_char) -> RequestId {
  send!(forward_report, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_academy_recruit_catalog(req: *const c_char) -> RequestId {
  send!(get_academy_recruit_catalog, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_armies(req: *const c_char) -> RequestId {
  send!(get_armies, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_army(req: *const c_char) -> RequestId {
  send!(get_army, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_army_owner(req: *const c_char) -> RequestId {
  send!(get_army_owner, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_bot_coords(req: *const c_char) -> RequestId {
  send!(get_bot_coords, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_chat_history(req: *const c_char) -> RequestId {
  send!(get_chat_history, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_cities(req: *const c_char) -> RequestId {
  send!(get_cities, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_city(req: *const c_char) -> RequestId {
  send!(get_city, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_city_score(req: *const c_char) -> RequestId {
  send!(get_city_score, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_continent_size(req: *const c_char) -> RequestId {
  send!(get_continent_size, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_idle_armies_at(req: *const c_char) -> RequestId {
  send!(get_idle_armies_at, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_idle_armies_coords(req: *const c_char) -> RequestId {
  send!(get_idle_armies_coords, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_maneuver(req: *const c_char) -> RequestId {
  send!(get_maneuver, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player(req: *const c_char) -> RequestId {
  send!(get_player, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_coords(req: *const c_char) -> RequestId {
  send!(get_player_coords, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_ids(req: *const c_char) -> RequestId {
  send!(get_player_ids, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_maintenance(req: *const c_char) -> RequestId {
  send!(get_player_maintenance, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_military(req: *const c_char) -> RequestId {
  send!(get_player_military, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_status(req: *const c_char) -> RequestId {
  send!(get_player_status, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_storage_capacity(req: *const c_char) -> RequestId {
  send!(get_player_storage_capacity, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_worlds(req: *const c_char) -> RequestId {
  send!(get_player_worlds, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_precursor_coords(req: *const c_char) -> RequestId {
  send!(get_precursor_coords, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_prefecture_build_catalog(req: *const c_char) -> RequestId {
  send!(get_prefecture_build_catalog, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_bot(req: *const c_char) -> RequestId {
  send!(get_public_bot, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_bots(req: *const c_char) -> RequestId {
  send!(get_public_bots, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_cities(req: *const c_char) -> RequestId {
  send!(get_public_cities, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_city(req: *const c_char) -> RequestId {
  send!(get_public_city, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_field(req: *const c_char) -> RequestId {
  send!(get_public_field, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_fields(req: *const c_char) -> RequestId {
  send!(get_public_fields, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_player(req: *const c_char) -> RequestId {
  send!(get_public_player, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_players(req: *const c_char) -> RequestId {
  send!(get_public_players, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_precursor(req: *const c_char) -> RequestId {
  send!(get_public_precursor, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_precursors(req: *const c_char) -> RequestId {
  send!(get_public_precursors, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_rank(req: *const c_char) -> RequestId {
  send!(get_rank, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_ranking(req: *const c_char) -> RequestId {
  send!(get_ranking, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world(req: *const c_char) -> RequestId {
  send!(get_remote_world, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world_limit() -> RequestId {
  send!(get_remote_world_limit)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world_limit_per_user() -> RequestId {
  send!(get_remote_world_limit_per_user)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_worlds() -> RequestId {
  send!(get_remote_worlds)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_round(req: *const c_char) -> RequestId {
  send!(get_round, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_server_kind() -> RequestId {
  send!(get_server_kind)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_stable_recruit_catalog(req: *const c_char) -> RequestId {
  send!(get_stable_recruit_catalog, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_workshop_recruit_catalog(req: *const c_char) -> RequestId {
  send!(get_workshop_recruit_catalog, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_bots(req: *const c_char) -> RequestId {
  send!(get_world_bots, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_config(req: *const c_char) -> RequestId {
  send!(get_world_config, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_personnel(req: *const c_char) -> RequestId {
  send!(get_world_personnel, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_players(req: *const c_char) -> RequestId {
  send!(get_world_players, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_precursors(req: *const c_char) -> RequestId {
  send!(get_world_precursors, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_stats(req: *const c_char) -> RequestId {
  send!(get_world_stats, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_player_exists(req: *const c_char) -> RequestId {
  send!(player_exists, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_push_chat_message(req: *const c_char) -> RequestId {
  send!(push_chat_message, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_rename_city(req: *const c_char) -> RequestId {
  send!(rename_city, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_request_maneuver(req: *const c_char) -> RequestId {
  send!(request_maneuver, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_save_local_world(req: *const c_char) -> RequestId {
  send!(save_local_world, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_search_city(req: *const c_char) -> RequestId {
  send!(search_city, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_search_public_city(req: *const c_char) -> RequestId {
  send!(search_public_city, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_server_version() -> RequestId {
  send!(version)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_set_player_ready(req: *const c_char) -> RequestId {
  send!(set_player_ready, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_set_player_status(req: *const c_char) -> RequestId {
  send!(set_player_status, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_simulate_battle(req: *const c_char) -> RequestId {
  send!(simulate_battle, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_spawn_player(req: *const c_char) -> RequestId {
  send!(spawn_player, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_start_round(req: *const c_char) -> RequestId {
  send!(start_round, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_toggle_building(req: *const c_char) -> RequestId {
  send!(toggle_building, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_user_exists(req: *const c_char) -> RequestId {
  send!(user_exists, req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_validate_token(req: *const c_char) -> RequestId {
  send!(validate_token, req)
}
