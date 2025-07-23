// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod chat;
mod cheat;
mod continent;
mod infrastructure;
mod player;
mod round;
mod script;
mod village;
mod world;

use crate::middleware::{CurrentPlayer, authorization};
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use crate::websocket::handle_socket;
use axum::extract::State;
use axum::extract::ws::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{any, get, post};
use axum::{Extension, Router, middleware};
use futures::TryFutureExt;
use infrastructure::prelude::*;
use nil_core::player::PlayerStatus;
use nil_core::world::World;

#[cfg(debug_assertions)]
use {
  tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
  tracing::Level,
};

pub(crate) fn create() -> Router<App> {
  #[rustfmt::skip]
  let router = Router::new()
    .route("/", get(ok))
    .route("/chat", get(chat::get))
    .route("/chat", post(chat::push))
    .route("/cheat/bot/spawn", get(cheat::spawn_bot))
    .route("/cheat/bot/{id}/infrastructure/storage", get(cheat::get_bot_storage_capacity))
    .route("/cheat/bot/{id}/resources", get(cheat::get_bot_resources))
    .route("/cheat/infrastructure", post(cheat::set_max_infrastructure))
    .route("/cheat/infrastructure/building", post(cheat::set_building_level))
    .route("/cheat/precursor/{id}/infrastructure/storage", get(cheat::get_precursor_storage_capacity))
    .route("/cheat/precursor/{id}/resources", get(cheat::get_precursor_resources))
    .route("/cheat/resources", get(cheat::set_max_resources))
    .route("/cheat/resources", post(cheat::set_resources))
    .route("/cheat/resources/food", get(cheat::set_max_food))
    .route("/cheat/resources/food", post(cheat::set_food))
    .route("/cheat/resources/iron", get(cheat::set_max_iron))
    .route("/cheat/resources/iron", post(cheat::set_iron))
    .route("/cheat/resources/silo", get(cheat::set_max_silo_resources))
    .route("/cheat/resources/stone", get(cheat::set_max_stone))
    .route("/cheat/resources/stone", post(cheat::set_stone))
    .route("/cheat/resources/warehouse", get(cheat::set_max_warehouse_resources))
    .route("/cheat/resources/wood", get(cheat::set_max_wood))
    .route("/cheat/resources/wood", post(cheat::set_wood))
    .route("/cheat/village/stability", post(cheat::set_stability))
    .route("/continent/field", post(continent::get_field))
    .route("/continent/fields", post(continent::get_fields))
    .route("/continent/size", get(continent::size))
    .route("/infrastructure/prefecture/build/add", post(prefecture::add_build_order))
    .route("/infrastructure/prefecture/build/cancel", post(prefecture::cancel_build_order))
    .route("/infrastructure/prefecture/build/catalog", post(prefecture::get_build_catalog))
    .route("/infrastructure/toggle", post(infrastructure::toggle))
    .route("/leave", get(leave))
    .route("/player", get(player::get_all))
    .route("/player", post(player::get))
    .route("/player/capacity", get(player::get_storage_capacity))
    .route("/player/maintenance", get(player::get_maintenance))
    .route("/player/spawn", post(player::spawn))
    .route("/player/status", post(player::set_status))
    .route("/player/{id}/coord", get(player::get_coords))
    .route("/player/{id}/exists", get(player::exists))
    .route("/player/{id}/status", get(player::get_status))
    .route("/round", get(round::get))
    .route("/round/start", get(round::start))
    .route("/round/turn/end", get(round::end_turn))
    .route("/script", get(script::get_all))
    .route("/script", post(script::add))
    .route("/script/chunk", post(script::execute_chunk))
    .route("/script/update", post(script::update))
    .route("/script/{id}", get(script::get))
    .route("/script/{id}/execute", get(script::execute))
    .route("/script/{id}/remove", get(script::remove))
    .route("/version", get(version))
    .route("/village", post(village::get))
    .route("/village/rename", post(village::rename))
    .route("/world/config", get(world::get_config))
    .route("/world/save", post(world::save))
    .route("/world/stats", get(world::get_stats))
    .route("/ws", any(websocket))
    .route_layer(middleware::from_fn(authorization));

  #[cfg(debug_assertions)]
  let router = router.layer(
    TraceLayer::new_for_http()
      .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
      .on_request(())
      .on_response(DefaultOnResponse::new().level(Level::TRACE))
      .on_failure(())
      .on_body_chunk(())
      .on_eos(()),
  );

  router
}

async fn websocket(
  ws: WebSocketUpgrade,
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
) -> Response {
  let player = current_player.0;
  let listener = app.world(World::subscribe).await;
  ws.on_upgrade(move |socket| handle_socket(socket, listener, player))
}

async fn ok() -> StatusCode {
  StatusCode::OK
}

async fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}

async fn leave(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
) -> Response {
  let id = &current_player.0;
  app
    .world_mut(|world| world.set_player_status(id, PlayerStatus::Inactive))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
