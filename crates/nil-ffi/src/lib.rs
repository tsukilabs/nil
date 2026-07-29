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
mod server;
mod status;

use client::CLIENT;
use futures::future::BoxFuture;
use json::serialize;
use nil_core::event::Event;
use std::ffi::{CString, c_char};
use std::ptr;
use std::sync::LazyLock;
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};

pub use request::RequestId;
pub use response::{Response, Result};
pub use status::Status;

use crate::server::SERVER;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
  RuntimeBuilder::new_multi_thread()
    .enable_all()
    .thread_name("libnil-worker")
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
pub unsafe extern "C" fn nil_ffi_poll(json_out: *mut *mut c_char) -> Status {
  if json_out.is_null() {
    return Status::ERR_NULL_POINTER;
  }

  unsafe { *json_out = ptr::null_mut() };

  match queue::poll() {
    Some(entry) => {
      match serialize(&entry) {
        Ok(json) => {
          let json = CString::new(json).unwrap();
          unsafe { *json_out = json.into_raw() };
          Status::OK
        }
        Err(_) => Status::ERR_SERIALIZATION,
      }
    }
    None => Status::ERR_NOTHING_TO_POLL,
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_ffi_shutdown() {
  queue::clear();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_ffi_version(request_id: RequestId) {
  push_ok!(request_id, env!("CARGO_PKG_VERSION"))
}

///////////////////////////////
//////////// SERVER ///////////
///////////////////////////////

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_is_host(request_id: RequestId) {
  async_push_ok!(request_id, SERVER.read().await.is_some())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_start_server(request_id: RequestId, json_options: *const c_char) {
  let f = |options| {
    RUNTIME.spawn(async move {
      let result = server::start_with_options(options).await;
      queue::push_result(request_id, Result::from(result));
    });
  };

  unsafe { json::with_ptr(request_id, json_options, f) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_start_server_with_savedata(
  request_id: RequestId,
  json_path: *const c_char,
) {
  let f = |path| {
    RUNTIME.spawn(async move {
      let result = server::start_with_savedata(path).await;
      queue::push_result(request_id, Result::from(result));
    });
  };

  unsafe { json::with_ptr(request_id, json_path, f) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_stop_server(request_id: RequestId) {
  RUNTIME.spawn(async move {
    server::stop().await;
    queue::push_ok(request_id, ());
  });
}

///////////////////////////////
//////////// CLIENT ///////////
///////////////////////////////

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_client_version(request_id: RequestId) {
  push_ok!(request_id, nil_client::VERSION)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_is_local(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.is_local())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_is_ready(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.is_ready().await)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_is_remote(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.is_remote())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_server_addr(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.server_addr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_set_user_agent(request_id: RequestId, json_user_agent: *const c_char) {
  let f = |user_agent: String| {
    RUNTIME.spawn(async move {
      CLIENT
        .write()
        .await
        .set_user_agent(&user_agent);

      queue::push_ok(request_id, ());
    });
  };

  unsafe { json::with_ptr(request_id, json_user_agent, f) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_stop_client(request_id: RequestId) {
  RUNTIME.spawn(async move {
    CLIENT.write().await.stop().await;
    queue::push_ok(request_id, ());
  });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_update_client(request_id: RequestId, json_options: *const c_char) {
  let f = |options| {
    type OnEvent = fn(Event) -> BoxFuture<'static, ()>;
    RUNTIME.spawn(async move {
      let result = CLIENT
        .write()
        .await
        .update(options, None::<OnEvent>)
        .await;

      queue::push_result(request_id, Result::from(result));
    });
  };

  unsafe { json::with_ptr(request_id, json_options, f) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_user_agent(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.user_agent().to_owned())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_world(request_id: RequestId) {
  RUNTIME.spawn(async move {
    match CLIENT.read().await.world() {
      Some(world) => queue::push_ok(request_id, world),
      None => queue::push_ok(request_id, None::<&str>),
    }
  });
}

///////////////////////////////
/////////// SERVER ////////////
///////////////////////////////

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_academy_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, add_academy_recruit_order, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_prefecture_build_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, add_prefecture_build_order, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_stable_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, add_stable_recruit_order, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_workshop_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, add_workshop_recruit_order, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_authorize(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, authorize, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_academy_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cancel_academy_recruit_order, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_maneuver(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cancel_maneuver, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_prefecture_build_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cancel_prefecture_build_order, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_stable_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cancel_stable_recruit_order, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_workshop_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cancel_workshop_recruit_order, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_fill_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_fill_world, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_academy_recruit_queue(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_academy_recruit_queue, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_academy_recruit_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_academy_recruit_queues, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_academy_recruit_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_all_academy_recruit_queues, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_prefecture_build_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_all_prefecture_build_queues, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_stable_recruit_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_all_stable_recruit_queues, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_build_steps(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_build_steps, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_cities(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_cities, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_city, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_ethics(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_ethics, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_idle_armies_at(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_idle_armies_at, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_idle_personnel_at(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_idle_personnel_at, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_infrastructure(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_infrastructure, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_maneuvers(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_maneuvers, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_maneuvers_of(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_maneuvers_of, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_player(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_player, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_players(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_players, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_prefecture_build_queue(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_prefecture_build_queue, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_prefecture_build_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_prefecture_build_queues, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_resources(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_resources, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_stable_recruit_queue(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_stable_recruit_queue, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_stable_recruit_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_stable_recruit_queues, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_storage_capacity(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_storage_capacity, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_bot_ethics(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_bot_ethics, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_building_level(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_building_level, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_food(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_food, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_iron(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_iron, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_food(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_max_food, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_infrastructure(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_max_infrastructure, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_iron(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_max_iron, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_resources(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_max_resources, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_silo_resources(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_max_silo_resources, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_stone(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_max_stone, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_warehouse_resources(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_max_warehouse_resources, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_wood(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_max_wood, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_resources(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_resources, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_stability(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_stability, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_stone(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_stone, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_wood(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_wood, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_skip_round(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_skip_round, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_bot(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_spawn_bot, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_spawn_city, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_personnel(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_spawn_personnel, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_create_remote_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, create_remote_world, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_create_user(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, create_user, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_delete_remote_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, delete_remote_world, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_forward_report(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, forward_report, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_academy_recruit_catalog(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_academy_recruit_catalog, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_armies(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_armies, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_army(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_army, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_army_owner(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_army_owner, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_bot_coords(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_bot_coords, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_chat_history(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_chat_history, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_cities(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_cities, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_city, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_city_score(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_city_score, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_continent_size(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_continent_size, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_idle_armies_at(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_idle_armies_at, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_idle_armies_coords(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_idle_armies_coords, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_maneuver(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_maneuver, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_coords(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_coords, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_ids(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_ids, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_maintenance(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_player_maintenance, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_military(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_military, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_status(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_status, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_storage_capacity(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_player_storage_capacity, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_worlds(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_worlds, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_precursor_coords(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_precursor_coords, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_prefecture_build_catalog(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_prefecture_build_catalog, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_bot(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_bot, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_bots(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_bots, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_cities(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_cities, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_city, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_field(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_field, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_fields(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_fields, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_player(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_player, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_players(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_players, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_precursor(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_precursor, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_precursors(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_precursors, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_rank(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_rank, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_ranking(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_ranking, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_remote_world, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world_limit(request_id: RequestId) {
  send!(request_id, get_remote_world_limit)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world_limit_per_user(request_id: RequestId) {
  send!(request_id, get_remote_world_limit_per_user)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_worlds(request_id: RequestId) {
  send!(request_id, get_remote_worlds)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_round(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_round, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_server_kind(request_id: RequestId) {
  send!(request_id, get_server_kind)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_stable_recruit_catalog(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_stable_recruit_catalog, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_workshop_recruit_catalog(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_workshop_recruit_catalog, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_bots(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_bots, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_config(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_config, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_personnel(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_personnel, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_players(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_players, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_precursors(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_precursors, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_stats(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_stats, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_player_exists(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, player_exists, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_push_chat_message(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, push_chat_message, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_rename_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, rename_city, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_request_maneuver(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, request_maneuver, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_save_local_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, save_local_world, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_search_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, search_city, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_search_public_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, search_public_city, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_server_version(request_id: RequestId) {
  send!(request_id, version)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_set_player_ready(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, set_player_ready, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_set_player_status(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, set_player_status, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_simulate_battle(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, simulate_battle, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_spawn_player(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, spawn_player, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_start_round(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, start_round, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_toggle_building(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, toggle_building, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_user_exists(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, user_exists, json_req)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_validate_token(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, validate_token, json_req)
}
