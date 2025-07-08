// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod chat;
mod cheat;
mod continent;
mod infrastructure;
mod lobby;
mod player;
mod round;
mod script;
mod village;
mod world;

use crate::error::CoreResult;
use crate::middleware::{CurrentPlayer, authorization};
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use crate::websocket::handle_socket;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{any, get, post};
use axum::{Extension, Router, middleware};
use infrastructure::prelude::*;
use nil_core::player::{PlayerId, PlayerStatus};
use nil_core::world::World;

#[cfg(debug_assertions)]
use {
  tower_http::trace::{
    DefaultMakeSpan,
    DefaultOnFailure,
    DefaultOnRequest,
    DefaultOnResponse,
    TraceLayer,
  },
  tracing::Level,
};

pub(crate) fn create() -> Router<App> {
  #[rustfmt::skip]
  let router = Router::new()
    .route("/", get(ok))
    .route("/chat", get(chat::get_all))
    .route("/chat/push", post(chat::push))
    .route("/cheat/infrastructure", post(cheat::set_max_infrastructure))
    .route("/cheat/infrastructure/building", post(cheat::set_building_level))
    .route("/cheat/resources", get(cheat::set_max_resources))
    .route("/cheat/resources", post(cheat::set_resources))
    .route("/cheat/resources/food", post(cheat::set_food))
    .route("/cheat/resources/iron", post(cheat::set_iron))
    .route("/cheat/resources/stone", post(cheat::set_stone))
    .route("/cheat/resources/wood", post(cheat::set_wood))
    .route("/cheat/village/stability", post(cheat::set_stability))
    .route("/continent/field", post(continent::get_field))
    .route("/continent/fields", post(continent::get_fields))
    .route("/continent/size", get(continent::size))
    .route("/infrastructure/prefecture/build/add", post(prefecture::add_build_order))
    .route("/infrastructure/prefecture/build/cancel", post(prefecture::cancel_build_order))
    .route("/infrastructure/prefecture/build/catalog", post(prefecture::get_build_catalog))
    .route("/leave", post(leave))
    .route("/lobby", get(lobby::get))
    .route("/lobby/ready", post(lobby::ready))
    .route("/player", get(player::get_all))
    .route("/player", post(player::get))
    .route("/player/coord", post(player::get_coords))
    .route("/player/exists", post(player::exists))
    .route("/player/remove-guest", post(player::remove_guest))
    .route("/player/set-status", post(player::set_status))
    .route("/player/spawn", post(player::spawn))
    .route("/player/spawn-village", post(player::spawn_village))
    .route("/round", get(round::get))
    .route("/round/end-turn", post(round::end_turn))
    .route("/round/start", get(round::start))
    .route("/script", post(script::get))
    .route("/script/add", post(script::add))
    .route("/script/add-many", post(script::add_many))
    .route("/script/all", post(script::get_all))
    .route("/script/remove", post(script::remove))
    .route("/script/update", post(script::update))
    .route("/version", get(version))
    .route("/village", post(village::get))
    .route("/world/config", get(world::get_config))
    .route("/world/save", post(world::save))
    .route("/world/stats", get(world::get_stats))
    .route("/ws", any(websocket))
    .route_layer(middleware::from_fn(authorization));

  #[cfg(debug_assertions)]
  let router = router.layer(
    TraceLayer::new_for_http()
      .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
      .on_request(DefaultOnRequest::new().level(Level::TRACE))
      .on_response(DefaultOnResponse::new().level(Level::TRACE))
      .on_failure(DefaultOnFailure::new().level(Level::ERROR))
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

async fn leave(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  let result: CoreResult<Response> = try {
    let mut world = app.world.write().await;
    let player = world.player(&id)?;
    if player.is_guest() {
      world.remove_guest(&id)?;
    } else if player.is_active() {
      world.set_player_status(&id, PlayerStatus::Inactive)?;
    }

    res!(OK)
  };

  result.unwrap_or_else(from_core_err)
}
