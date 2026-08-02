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
mod runtime;
mod server;
mod status;

use crate::runtime::RUNTIME;
use crate::server::SERVER;
use client::CLIENT;
use futures::future::BoxFuture;
use json::serialize;
use nil_core::event::Event;
use std::ffi::{CString, c_char};
use std::ptr;
use tap::TryConv;

pub use request::RequestId;
pub use response::{Response, Result};
pub use status::Status;

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
  push_ok!(request_id, env!("CARGO_PKG_VERSION"));
}

///////////////////////////////
//////////// SERVER ///////////
///////////////////////////////

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_is_host(request_id: RequestId) {
  async_push_ok!(request_id, SERVER.read().await.is_some());
}

/// [`nil_server::local::start`]
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

/// [`nil_server::local::load`]
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

/// [`LocalServer::stop`](nil_server::local::LocalServer::stop)
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
  push_ok!(request_id, nil_client::VERSION);
}

/// [`Client::is_local`](nil_client::Client::is_local)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_is_local(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.is_local());
}

/// [`Client::is_ready`](nil_client::Client::is_ready)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_is_ready(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.is_ready().await);
}

/// [`Client::is_remote`](nil_client::Client::is_remote)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_is_remote(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.is_remote());
}

/// [`Client::server_addr`](nil_client::Client::server_addr)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_server_addr(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.server_addr());
}

/// [`Client::set_user_agent`](nil_client::Client::set_user_agent)
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

/// [`Client::stop`](nil_client::Client::stop)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_stop_client(request_id: RequestId) {
  RUNTIME.spawn(async move {
    CLIENT.write().await.stop().await;
    queue::push_ok(request_id, ());
  });
}

/// [`Client::update`](nil_client::Client::update)
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

/// [`Client::user_agent`](nil_client::Client::user_agent)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_user_agent(request_id: RequestId) {
  async_push_ok!(request_id, CLIENT.read().await.user_agent().to_owned());
}

/// [`Client::world`](nil_client::Client::world)
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
/////////// RUNTIME ///////////
///////////////////////////////

/// [`RuntimeMetrics::global_queue_depth`](tokio::runtime::RuntimeMetrics::global_queue_depth)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_runtime_global_queue_depth(request_id: RequestId) {
  push_result!(
    request_id,
    RUNTIME
      .metrics()
      .global_queue_depth()
      .try_conv::<u32>()
  );
}

/// [`RuntimeMetrics::num_alive_tasks`](tokio::runtime::RuntimeMetrics::num_alive_tasks)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_runtime_num_alive_tasks(request_id: RequestId) {
  push_result!(
    request_id,
    RUNTIME
      .metrics()
      .num_alive_tasks()
      .try_conv::<u32>()
  );
}

/// [`RuntimeMetrics::num_workers`](tokio::runtime::RuntimeMetrics::num_workers)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_runtime_num_workers(request_id: RequestId) {
  push_result!(
    request_id,
    RUNTIME
      .metrics()
      .num_workers()
      .try_conv::<u32>()
  );
}

///////////////////////////////
////////// ENDPOINTS //////////
///////////////////////////////

/// [`Client::add_academy_recruit_order`](nil_client::Client::add_academy_recruit_order)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_academy_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, add_academy_recruit_order, json_req);
}

/// [`Client::add_prefecture_build_order`](nil_client::Client::add_prefecture_build_order)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_prefecture_build_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, add_prefecture_build_order, json_req);
}

/// [`Client::add_stable_recruit_order`](nil_client::Client::add_stable_recruit_order)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_stable_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, add_stable_recruit_order, json_req);
}

/// [`Client::add_workshop_recruit_order`](nil_client::Client::add_workshop_recruit_order)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_add_workshop_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, add_workshop_recruit_order, json_req);
}

/// [`Client::authorize`](nil_client::Client::authorize)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_authorize(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, authorize, json_req);
}

/// [`Client::cancel_academy_recruit_order`](nil_client::Client::cancel_academy_recruit_order)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_academy_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cancel_academy_recruit_order, json_req);
}

/// [`Client::cancel_maneuver`](nil_client::Client::cancel_maneuver)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_maneuver(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cancel_maneuver, json_req);
}

/// [`Client::cancel_prefecture_build_order`](nil_client::Client::cancel_prefecture_build_order)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_prefecture_build_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cancel_prefecture_build_order, json_req);
}

/// [`Client::cancel_stable_recruit_order`](nil_client::Client::cancel_stable_recruit_order)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_stable_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cancel_stable_recruit_order, json_req);
}

/// [`Client::cancel_workshop_recruit_order`](nil_client::Client::cancel_workshop_recruit_order)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cancel_workshop_recruit_order(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cancel_workshop_recruit_order, json_req);
}

/// [`Client::cheat_fill_world`](nil_client::Client::cheat_fill_world)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_fill_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_fill_world, json_req);
}

/// [`Client::cheat_get_academy_recruit_queue`](nil_client::Client::cheat_get_academy_recruit_queue)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_academy_recruit_queue(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_academy_recruit_queue, json_req);
}

/// [`Client::cheat_get_academy_recruit_queues`](nil_client::Client::cheat_get_academy_recruit_queues)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_academy_recruit_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_academy_recruit_queues, json_req);
}

/// [`Client::cheat_get_all_academy_recruit_queues`](nil_client::Client::cheat_get_all_academy_recruit_queues)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_academy_recruit_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_all_academy_recruit_queues, json_req);
}

/// [`Client::cheat_get_all_prefecture_build_queues`](nil_client::Client::cheat_get_all_prefecture_build_queues)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_prefecture_build_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_all_prefecture_build_queues, json_req);
}

/// [`Client::cheat_get_all_stable_recruit_queues`](nil_client::Client::cheat_get_all_stable_recruit_queues)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_all_stable_recruit_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_all_stable_recruit_queues, json_req);
}

/// [`Client::cheat_get_build_steps`](nil_client::Client::cheat_get_build_steps)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_build_steps(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_build_steps, json_req);
}

/// [`Client::cheat_get_cities`](nil_client::Client::cheat_get_cities)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_cities(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_cities, json_req);
}

/// [`Client::cheat_get_city`](nil_client::Client::cheat_get_city)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_city, json_req);
}

/// [`Client::cheat_get_ethics`](nil_client::Client::cheat_get_ethics)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_ethics(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_ethics, json_req);
}

/// [`Client::cheat_get_idle_armies_at`](nil_client::Client::cheat_get_idle_armies_at)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_idle_armies_at(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_idle_armies_at, json_req);
}

/// [`Client::cheat_get_idle_personnel_at`](nil_client::Client::cheat_get_idle_personnel_at)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_idle_personnel_at(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_idle_personnel_at, json_req);
}

/// [`Client::cheat_get_infrastructure`](nil_client::Client::cheat_get_infrastructure)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_infrastructure(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_infrastructure, json_req);
}

/// [`Client::cheat_get_maneuvers`](nil_client::Client::cheat_get_maneuvers)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_maneuvers(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_maneuvers, json_req);
}

/// [`Client::cheat_get_maneuvers_of`](nil_client::Client::cheat_get_maneuvers_of)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_maneuvers_of(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_maneuvers_of, json_req);
}

/// [`Client::cheat_get_player`](nil_client::Client::cheat_get_player)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_player(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_player, json_req);
}

/// [`Client::cheat_get_players`](nil_client::Client::cheat_get_players)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_players(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_players, json_req);
}

/// [`Client::cheat_get_prefecture_build_queue`](nil_client::Client::cheat_get_prefecture_build_queue)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_prefecture_build_queue(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_prefecture_build_queue, json_req);
}

/// [`Client::cheat_get_prefecture_build_queues`](nil_client::Client::cheat_get_prefecture_build_queues)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_prefecture_build_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_prefecture_build_queues, json_req);
}

/// [`Client::cheat_get_resources`](nil_client::Client::cheat_get_resources)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_resources(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_get_resources, json_req);
}

/// [`Client::cheat_get_stable_recruit_queue`](nil_client::Client::cheat_get_stable_recruit_queue)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_stable_recruit_queue(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_stable_recruit_queue, json_req);
}

/// [`Client::cheat_get_stable_recruit_queues`](nil_client::Client::cheat_get_stable_recruit_queues)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_stable_recruit_queues(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_stable_recruit_queues, json_req);
}

/// [`Client::cheat_get_storage_capacity`](nil_client::Client::cheat_get_storage_capacity)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_get_storage_capacity(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_get_storage_capacity, json_req);
}

/// [`Client::cheat_set_bot_ethics`](nil_client::Client::cheat_set_bot_ethics)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_bot_ethics(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_bot_ethics, json_req);
}

/// [`Client::cheat_set_building_level`](nil_client::Client::cheat_set_building_level)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_building_level(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_building_level, json_req);
}

/// [`Client::cheat_set_food`](nil_client::Client::cheat_set_food)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_food(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_food, json_req);
}

/// [`Client::cheat_set_iron`](nil_client::Client::cheat_set_iron)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_iron(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_iron, json_req);
}

/// [`Client::cheat_set_market_fee`](nil_client::Client::cheat_set_market_fee)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_market_fee(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_market_fee, json_req);
}

/// [`Client::cheat_set_max_food`](nil_client::Client::cheat_set_max_food)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_food(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_max_food, json_req);
}

/// [`Client::cheat_set_max_infrastructure`](nil_client::Client::cheat_set_max_infrastructure)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_infrastructure(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_max_infrastructure, json_req);
}

/// [`Client::cheat_set_max_iron`](nil_client::Client::cheat_set_max_iron)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_iron(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_max_iron, json_req);
}

/// [`Client::cheat_set_max_resources`](nil_client::Client::cheat_set_max_resources)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_resources(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_max_resources, json_req);
}

/// [`Client::cheat_set_max_silo_resources`](nil_client::Client::cheat_set_max_silo_resources)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_silo_resources(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_max_silo_resources, json_req);
}

/// [`Client::cheat_set_max_stone`](nil_client::Client::cheat_set_max_stone)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_stone(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_max_stone, json_req);
}

/// [`Client::cheat_set_max_warehouse_resources`](nil_client::Client::cheat_set_max_warehouse_resources)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_warehouse_resources(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, cheat_set_max_warehouse_resources, json_req);
}

/// [`Client::cheat_set_max_wood`](nil_client::Client::cheat_set_max_wood)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_max_wood(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_max_wood, json_req);
}

/// [`Client::cheat_set_resources`](nil_client::Client::cheat_set_resources)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_resources(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_resources, json_req);
}

/// [`Client::cheat_set_stability`](nil_client::Client::cheat_set_stability)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_stability(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_stability, json_req);
}

/// [`Client::cheat_set_stone`](nil_client::Client::cheat_set_stone)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_stone(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_stone, json_req);
}

/// [`Client::cheat_set_wood`](nil_client::Client::cheat_set_wood)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_set_wood(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_set_wood, json_req);
}

/// [`Client::cheat_skip_round`](nil_client::Client::cheat_skip_round)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_skip_round(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_skip_round, json_req);
}

/// [`Client::cheat_spawn_bot`](nil_client::Client::cheat_spawn_bot)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_bot(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_spawn_bot, json_req);
}

/// [`Client::cheat_spawn_city`](nil_client::Client::cheat_spawn_city)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_spawn_city, json_req);
}

/// [`Client::cheat_spawn_personnel`](nil_client::Client::cheat_spawn_personnel)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_cheat_spawn_personnel(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, cheat_spawn_personnel, json_req);
}

/// [`Client::create_remote_world`](nil_client::Client::create_remote_world)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_create_remote_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, create_remote_world, json_req);
}

/// [`Client::create_user`](nil_client::Client::create_user)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_create_user(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, create_user, json_req);
}

/// [`Client::delete_remote_world`](nil_client::Client::delete_remote_world)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_delete_remote_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, delete_remote_world, json_req);
}

/// [`Client::forward_report`](nil_client::Client::forward_report)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_forward_report(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, forward_report, json_req);
}

/// [`Client::get_academy_recruit_catalog`](nil_client::Client::get_academy_recruit_catalog)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_academy_recruit_catalog(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_academy_recruit_catalog, json_req);
}

/// [`Client::get_armies`](nil_client::Client::get_armies)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_armies(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_armies, json_req);
}

/// [`Client::get_army`](nil_client::Client::get_army)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_army(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_army, json_req);
}

/// [`Client::get_army_owner`](nil_client::Client::get_army_owner)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_army_owner(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_army_owner, json_req);
}

/// [`Client::get_bot_coords`](nil_client::Client::get_bot_coords)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_bot_coords(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_bot_coords, json_req);
}

/// [`Client::get_chat_history`](nil_client::Client::get_chat_history)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_chat_history(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_chat_history, json_req);
}

/// [`Client::get_cities`](nil_client::Client::get_cities)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_cities(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_cities, json_req);
}

/// [`Client::get_city`](nil_client::Client::get_city)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_city, json_req);
}

/// [`Client::get_city_score`](nil_client::Client::get_city_score)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_city_score(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_city_score, json_req);
}

/// [`Client::get_continent_size`](nil_client::Client::get_continent_size)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_continent_size(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_continent_size, json_req);
}

/// [`Client::get_idle_armies_at`](nil_client::Client::get_idle_armies_at)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_idle_armies_at(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_idle_armies_at, json_req);
}

/// [`Client::get_idle_armies_coords`](nil_client::Client::get_idle_armies_coords)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_idle_armies_coords(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_idle_armies_coords, json_req);
}

/// [`Client::get_maneuver`](nil_client::Client::get_maneuver)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_maneuver(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_maneuver, json_req);
}

/// [`Client::get_market_fee`](nil_client::Client::get_market_fee)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_market_fee(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_market_fee, json_req);
}

/// [`Client::get_player`](nil_client::Client::get_player)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player, json_req);
}

/// [`Client::get_player_coords`](nil_client::Client::get_player_coords)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_coords(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_coords, json_req);
}

/// [`Client::get_player_ids`](nil_client::Client::get_player_ids)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_ids(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_ids, json_req);
}

/// [`Client::get_player_maintenance`](nil_client::Client::get_player_maintenance)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_maintenance(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_player_maintenance, json_req);
}

/// [`Client::get_player_military`](nil_client::Client::get_player_military)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_military(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_military, json_req);
}

/// [`Client::get_player_status`](nil_client::Client::get_player_status)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_status(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_status, json_req);
}

/// [`Client::get_player_storage_capacity`](nil_client::Client::get_player_storage_capacity)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_storage_capacity(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_player_storage_capacity, json_req);
}

/// [`Client::get_player_worlds`](nil_client::Client::get_player_worlds)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_player_worlds(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_player_worlds, json_req);
}

/// [`Client::get_precursor_coords`](nil_client::Client::get_precursor_coords)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_precursor_coords(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_precursor_coords, json_req);
}

/// [`Client::get_prefecture_build_catalog`](nil_client::Client::get_prefecture_build_catalog)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_prefecture_build_catalog(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_prefecture_build_catalog, json_req);
}

/// [`Client::get_public_bot`](nil_client::Client::get_public_bot)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_bot(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_bot, json_req);
}

/// [`Client::get_public_bots`](nil_client::Client::get_public_bots)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_bots(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_bots, json_req);
}

/// [`Client::get_public_cities`](nil_client::Client::get_public_cities)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_cities(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_cities, json_req);
}

/// [`Client::get_public_city`](nil_client::Client::get_public_city)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_city, json_req);
}

/// [`Client::get_public_field`](nil_client::Client::get_public_field)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_field(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_field, json_req);
}

/// [`Client::get_public_fields`](nil_client::Client::get_public_fields)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_fields(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_fields, json_req);
}

/// [`Client::get_public_player`](nil_client::Client::get_public_player)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_player(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_player, json_req);
}

/// [`Client::get_public_players`](nil_client::Client::get_public_players)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_players(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_players, json_req);
}

/// [`Client::get_public_precursor`](nil_client::Client::get_public_precursor)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_precursor(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_precursor, json_req);
}

/// [`Client::get_public_precursors`](nil_client::Client::get_public_precursors)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_public_precursors(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_public_precursors, json_req);
}

/// [`Client::get_rank`](nil_client::Client::get_rank)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_rank(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_rank, json_req);
}

/// [`Client::get_ranking`](nil_client::Client::get_ranking)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_ranking(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_ranking, json_req);
}

/// [`Client::get_remote_world`](nil_client::Client::get_remote_world)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_remote_world, json_req);
}

/// [`Client::get_remote_world_limit`](nil_client::Client::get_remote_world_limit)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world_limit(request_id: RequestId) {
  send!(request_id, get_remote_world_limit);
}

/// [`Client::get_remote_world_limit_per_user`](nil_client::Client::get_remote_world_limit_per_user)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_world_limit_per_user(request_id: RequestId) {
  send!(request_id, get_remote_world_limit_per_user);
}

/// [`Client::get_remote_worlds`](nil_client::Client::get_remote_worlds)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_remote_worlds(request_id: RequestId) {
  send!(request_id, get_remote_worlds);
}

/// [`Client::get_round`](nil_client::Client::get_round)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_round(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_round, json_req);
}

/// [`Client::get_server_kind`](nil_client::Client::get_server_kind)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_server_kind(request_id: RequestId) {
  send!(request_id, get_server_kind);
}

/// [`Client::get_stable_recruit_catalog`](nil_client::Client::get_stable_recruit_catalog)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_stable_recruit_catalog(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_stable_recruit_catalog, json_req);
}

/// [`Client::get_workshop_recruit_catalog`](nil_client::Client::get_workshop_recruit_catalog)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_workshop_recruit_catalog(
  request_id: RequestId,
  json_req: *const c_char,
) {
  send!(request_id, get_workshop_recruit_catalog, json_req);
}

/// [`Client::get_world_bots`](nil_client::Client::get_world_bots)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_bots(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_bots, json_req);
}

/// [`Client::get_world_config`](nil_client::Client::get_world_config)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_config(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_config, json_req);
}

/// [`Client::get_world_personnel`](nil_client::Client::get_world_personnel)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_personnel(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_personnel, json_req);
}

/// [`Client::get_world_players`](nil_client::Client::get_world_players)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_players(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_players, json_req);
}

/// [`Client::get_world_precursors`](nil_client::Client::get_world_precursors)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_precursors(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_precursors, json_req);
}

/// [`Client::get_world_stats`](nil_client::Client::get_world_stats)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_get_world_stats(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, get_world_stats, json_req);
}

/// [`Client::player_exists`](nil_client::Client::player_exists)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_player_exists(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, player_exists, json_req);
}

/// [`Client::push_chat_message`](nil_client::Client::push_chat_message)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_push_chat_message(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, push_chat_message, json_req);
}

/// [`Client::rename_city`](nil_client::Client::rename_city)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_rename_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, rename_city, json_req);
}

/// [`Client::request_maneuver`](nil_client::Client::request_maneuver)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_request_maneuver(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, request_maneuver, json_req);
}

/// [`Client::save_local_world`](nil_client::Client::save_local_world)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_save_local_world(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, save_local_world, json_req);
}

/// [`Client::search_city`](nil_client::Client::search_city)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_search_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, search_city, json_req);
}

/// [`Client::search_public_city`](nil_client::Client::search_public_city)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_search_public_city(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, search_public_city, json_req);
}

/// [`Client::send_resources`](nil_client::Client::send_resources)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_send_resources(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, send_resources, json_req);
}

/// [`Client::version`](nil_client::Client::version)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_server_version(request_id: RequestId) {
  send!(request_id, version);
}

/// [`Client::set_player_ready`](nil_client::Client::set_player_ready)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_set_player_ready(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, set_player_ready, json_req);
}

/// [`Client::set_player_status`](nil_client::Client::set_player_status)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_set_player_status(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, set_player_status, json_req);
}

/// [`Client::simulate_battle`](nil_client::Client::simulate_battle)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_simulate_battle(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, simulate_battle, json_req);
}

/// [`Client::spawn_player`](nil_client::Client::spawn_player)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_spawn_player(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, spawn_player, json_req);
}

/// [`Client::start_round`](nil_client::Client::start_round)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_start_round(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, start_round, json_req);
}

/// [`Client::toggle_building`](nil_client::Client::toggle_building)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_toggle_building(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, toggle_building, json_req);
}

/// [`Client::user_exists`](nil_client::Client::user_exists)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_user_exists(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, user_exists, json_req);
}

/// [`Client::validate_token`](nil_client::Client::validate_token)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nil_validate_token(request_id: RequestId, json_req: *const c_char) {
  send!(request_id, validate_token, json_req);
}
