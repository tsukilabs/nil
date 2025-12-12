// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod battle;
mod chat;
mod cheat;
mod city;
mod continent;
mod infrastructure;
mod military;
mod npc;
mod player;
mod ranking;
mod report;
mod round;
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
    .route("/add-academy-recruit-order", post(academy::add_recruit_order))
    .route("/add-prefecture-build-order", post(prefecture::add_build_order))
    .route("/add-stable-recruit-order", post(stable::add_recruit_order))
    .route("/cancel-academy-recruit-order", post(academy::cancel_recruit_order))
    .route("/cancel-prefecture-build-order", post(prefecture::cancel_build_order))
    .route("/cancel-stable-recruit-order", post(stable::cancel_recruit_order))
    .route("/cheat-get-academy-recruit-queue", post(cheat::infrastructure::get_academy_recruit_queue))
    .route("/cheat-get-build-steps", post(cheat::behavior::get_build_steps))
    .route("/cheat-get-ethics", post(cheat::npc::get_ethics))
    .route("/cheat-get-infrastructure", post(cheat::infrastructure::get_infrastructure))
    .route("/cheat-get-prefecture-build-queue", post(cheat::infrastructure::get_prefecture_build_queue))
    .route("/cheat-get-resources", post(cheat::resources::get_resources))
    .route("/cheat-skip-round", post(cheat::round::skip))
    .route("/cheat-get-stable-recruit-queue", post(cheat::infrastructure::get_stable_recruit_queue))
    .route("/cheat-get-storage-capacity", post(cheat::infrastructure::get_storage_capacity))
    .route("/cheat-set-bot-ethics", post(cheat::npc::set_bot_ethics))
    .route("/cheat-set-building-level", post(cheat::infrastructure::set_building_level))
    .route("/cheat-set-food", post(cheat::resources::set_food))
    .route("/cheat-set-iron", post(cheat::resources::set_iron))
    .route("/cheat-set-max-food", post(cheat::resources::set_max_food))
    .route("/cheat-set-max-infrastructure", post(cheat::infrastructure::set_max_infrastructure))
    .route("/cheat-set-max-iron", post(cheat::resources::set_max_iron))
    .route("/cheat-set-max-resources", post(cheat::resources::set_max_resources))
    .route("/cheat-set-max-silo-resources", post(cheat::resources::set_max_silo_resources))
    .route("/cheat-set-max-stone", post(cheat::resources::set_max_stone))
    .route("/cheat-set-max-warehouse-resources", post(cheat::resources::set_max_warehouse_resources))
    .route("/cheat-set-max-wood", post(cheat::resources::set_max_wood))
    .route("/cheat-set-resources", post(cheat::resources::set_resources))
    .route("/cheat-set-stability", post(cheat::city::set_stability))
    .route("/cheat-set-stone", post(cheat::resources::set_stone))
    .route("/cheat-set-wood", post(cheat::resources::set_wood))
    .route("/cheat-spawn-bot", post(cheat::npc::spawn_bot))
    .route("/cheat-spawn-personnel", post(cheat::military::spawn_personnel))
    .route("/get-academy-recruit-catalog", post(academy::get_recruit_catalog))
    .route("/get-bot-coords", post(npc::bot::get_coords))
    .route("/get-chat-history", get(chat::get))
    .route("/get-city", post(city::get))
    .route("/get-city-score", post(city::get_score))
    .route("/get-continent-size", get(continent::size))
    .route("/get-field", post(continent::get_field))
    .route("/get-fields", post(continent::get_fields))
    .route("/get-player", post(player::get))
    .route("/get-player-coords", post(player::get_coords))
    .route("/get-player-maintenance", get(player::get_maintenance))
    .route("/get-player-military", get(player::get_military))
    .route("/get-player-reports", get(player::get_reports))
    .route("/get-player-status", post(player::get_status))
    .route("/get-player-storage-capacity", get(player::get_storage_capacity))
    .route("/get-players", get(player::get_all))
    .route("/get-precursor-coords", post(npc::precursor::get_coords))
    .route("/get-prefecture-build-catalog", post(prefecture::get_build_catalog))
    .route("/get-public-bot", post(npc::bot::get_public))
    .route("/get-public-city", post(city::get_public))
    .route("/get-public-player", post(player::get_public))
    .route("/get-public-players", get(player::get_all_public))
    .route("/get-public-precursor", post(npc::precursor::get_public))
    .route("/get-rank", post(ranking::get_rank))
    .route("/get-ranking", get(ranking::get))
    .route("/get-report", post(report::get))
    .route("/get-reports", post(report::get_by))
    .route("/get-round", get(round::get))
    .route("/get-stable-recruit-catalog", post(stable::get_recruit_catalog))
    .route("/get-world-config", get(world::get_config))
    .route("/get-world-stats", get(world::get_stats))
    .route("/leave", get(leave))
    .route("/player-exists", post(player::exists))
    .route("/push-chat-message", post(chat::push))
    .route("/rename-city", post(city::rename))
    .route("/request-maneuver", post(military::request_maneuver))
    .route("/save-world", post(world::save))
    .route("/search-city", post(city::search))
    .route("/search-public-city", post(city::search_public))
    .route("/set-player-ready", post(round::set_ready))
    .route("/set-player-status", post(player::set_status))
    .route("/simulate-battle", post(battle::simulate))
    .route("/spawn-player", post(player::spawn))
    .route("/start-round", get(round::start))
    .route("/toggle-building", post(infrastructure::toggle))
    .route("/version", get(version))
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
