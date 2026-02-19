// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![expect(clippy::wildcard_imports)]

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
mod user;
mod world;

use crate::app::App;
use crate::error::Error;
use crate::middleware::authorization::{CurrentPlayer, authorization, decode_jwt, encode_jwt};
use crate::res;
use crate::response::from_database_err;
use crate::websocket::handle_socket;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Extension, Json, Query, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{any, get, post};
use axum::{Router, middleware};
use nil_core::player::PlayerId;
use nil_core::world::World;
use nil_payload::{AuthorizeRequest, ValidateTokenRequest, WebsocketQuery};
use nil_server_types::ServerKind;
use tokio::task::spawn_blocking;
use tower_http::trace::{
  DefaultMakeSpan,
  DefaultOnFailure,
  DefaultOnRequest,
  DefaultOnResponse,
  TraceLayer,
};
use tracing::Level;

#[expect(clippy::too_many_lines)]
pub(crate) fn create() -> Router<App> {
  #[rustfmt::skip]
  let router = Router::new()
    .route("/add-academy-recruit-order", post(infrastructure::academy::add_recruit_order))
    .route("/add-prefecture-build-order", post(infrastructure::prefecture::add_build_order))
    .route("/add-stable-recruit-order", post(infrastructure::stable::add_recruit_order))
    .route("/add-workshop-recruit-order", post(infrastructure::workshop::add_recruit_order))
    .route("/cancel-academy-recruit-order", post(infrastructure::academy::cancel_recruit_order))
    .route("/cancel-prefecture-build-order", post(infrastructure::prefecture::cancel_build_order))
    .route("/cancel-stable-recruit-order", post(infrastructure::stable::cancel_recruit_order))
    .route("/cancel-workshop-recruit-order", post(infrastructure::workshop::cancel_recruit_order))
    .route("/cheat-get-academy-recruit-queue", post(cheat::infrastructure::get_academy_recruit_queue))
    .route("/cheat-get-academy-recruit-queues", post(cheat::infrastructure::get_academy_recruit_queues))
    .route("/cheat-get-all-academy-recruit-queues", post(cheat::infrastructure::get_all_academy_recruit_queues))
    .route("/cheat-get-all-prefecture-build-queues", post(cheat::infrastructure::get_all_prefecture_build_queues))
    .route("/cheat-get-all-stable-recruit-queues", post(cheat::infrastructure::get_all_stable_recruit_queues))
    .route("/cheat-get-build-steps", post(cheat::behavior::get_build_steps))
    .route("/cheat-get-ethics", post(cheat::npc::get_ethics))
    .route("/cheat-get-idle-armies-at", post(cheat::military::get_idle_armies_at))
    .route("/cheat-get-idle-personnel-at", post(cheat::military::get_idle_personnel_at))
    .route("/cheat-get-infrastructure", post(cheat::infrastructure::get_infrastructure))
    .route("/cheat-get-prefecture-build-queue", post(cheat::infrastructure::get_prefecture_build_queue))
    .route("/cheat-get-prefecture-build-queues", post(cheat::infrastructure::get_prefecture_build_queues))
    .route("/cheat-get-resources", post(cheat::resources::get_resources))
    .route("/cheat-get-stable-recruit-queue", post(cheat::infrastructure::get_stable_recruit_queue))
    .route("/cheat-get-stable-recruit-queues", post(cheat::infrastructure::get_stable_recruit_queues))
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
    .route("/cheat-skip-round", post(cheat::round::skip))
    .route("/cheat-spawn-bot", post(cheat::npc::spawn_bot))
    .route("/cheat-spawn-personnel", post(cheat::military::spawn_personnel))
    .route("/create-remote-world", post(world::remote::create))
    .route("/get-academy-recruit-catalog", post(infrastructure::academy::get_recruit_catalog))
    .route("/get-chat-history", post(chat::get))
    .route("/get-city", post(city::get_city))
    .route("/get-player", post(player::get_player))
    .route("/get-player-maintenance", post(player::get_maintenance))
    .route("/get-player-military", post(player::get_military))
    .route("/get-player-reports", post(player::get_reports))
    .route("/get-player-storage-capacity", post(player::get_storage_capacity))
    .route("/get-prefecture-build-catalog", post(infrastructure::prefecture::get_build_catalog))
    .route("/get-report", post(report::get))
    .route("/get-reports", post(report::get_by))
    .route("/get-stable-recruit-catalog", post(infrastructure::stable::get_recruit_catalog))
    .route("/get-workshop-recruit-catalog", post(infrastructure::workshop::get_recruit_catalog))
    .route("/leave", post(world::leave))
    .route("/push-chat-message", post(chat::push))
    .route("/push-stdout-message", post(chat::push_stdout))
    .route("/rename-city", post(city::rename_city))
    .route("/request-maneuver", post(military::request_maneuver))
    .route("/save-local-world", post(world::local::save))
    .route("/search-city", post(city::search_city))
    .route("/set-player-ready", post(round::set_ready))
    .route("/set-player-status", post(player::set_status))
    .route("/spawn-player", post(player::spawn))
    .route("/start-round", post(round::start))
    .route("/toggle-building", post(infrastructure::toggle))
    .route("/websocket", any(websocket))
    .route_layer(middleware::from_fn(authorization))

    // These don't need authorization.
    .route("/", get(ok))
    .route("/authorize", post(authorize))
    .route("/create-user", post(user::create))
    .route("/get-bot-coords", post(npc::bot::get_coords))
    .route("/get-city-score", post(city::get_city_score))
    .route("/get-continent-size", post(continent::size))
    .route("/get-player-coords", post(player::get_coords))
    .route("/get-player-status", post(player::get_status))
    .route("/get-player-worlds", post(player::get_worlds))
    .route("/get-precursor-coords", post(npc::precursor::get_coords))
    .route("/get-public-bot", post(npc::bot::get_public_bot))
    .route("/get-public-bots", post(npc::bot::get_public_bots))
    .route("/get-public-cities", post(city::get_public_cities))
    .route("/get-public-city", post(city::get_public_city))
    .route("/get-public-field", post(continent::get_public_field))
    .route("/get-public-fields", post(continent::get_public_fields))
    .route("/get-public-player", post(player::get_public_player))
    .route("/get-public-players", post(player::get_public_players))
    .route("/get-public-precursor", post(npc::precursor::get_public_precursor))
    .route("/get-public-precursors", post(npc::precursor::get_public_precursors))
    .route("/get-rank", post(ranking::get_rank))
    .route("/get-ranking", post(ranking::get))
    .route("/get-remote-world", post(world::remote::get))
    .route("/get-remote-worlds", get(world::remote::get_all))
    .route("/get-round", post(round::get))
    .route("/get-server-kind", get(server_kind))
    .route("/get-world-bots", post(world::get_bots))
    .route("/get-world-config", post(world::get_config))
    .route("/get-world-players", post(world::get_players))
    .route("/get-world-precursors", post(world::get_precursors))
    .route("/get-world-stats", post(world::get_stats))
    .route("/player-exists", post(player::exists))
    .route("/search-public-city", post(city::search_public_city))
    .route("/simulate-battle", post(battle::simulate))
    .route("/user-exists", post(user::exists))
    .route("/validate-token", post(validate_token))
    .route("/version", get(version))
    
    // Files.
    .route("/robots.txt", any(robots_txt));

  router.layer(
    TraceLayer::new_for_http()
      .make_span_with(DefaultMakeSpan::new().include_headers(true))
      .on_request(DefaultOnRequest::new().level(Level::TRACE))
      .on_response(DefaultOnResponse::new().level(Level::TRACE))
      .on_failure(DefaultOnFailure::new().level(Level::TRACE))
      .on_body_chunk(())
      .on_eos(()),
  )
}

async fn authorize(State(app): State<App>, Json(req): Json<AuthorizeRequest>) -> Response {
  let Ok(result) = spawn_blocking(move || {
    match app.server_kind() {
      ServerKind::Local { .. } => encode_jwt(req.player),
      ServerKind::Remote => {
        let Some(password) = req.password else {
          return Err(Error::MissingPassword);
        };

        if app
          .database()
          .get_user(&req.player)
          .map_err(Into::<Error>::into)?
          .verify_password(&password)
        {
          encode_jwt(req.player)
        } else {
          Err(Error::IncorrectUserCredentials)
        }
      }
    }
  })
  .await
  else {
    return res!(INTERNAL_SERVER_ERROR);
  };

  result
    .map(|token| res!(OK, Json(token)))
    .unwrap_or_else(Response::from)
}

async fn ok() -> StatusCode {
  StatusCode::OK
}

async fn robots_txt() -> &'static str {
  "User-agent: *\nDisallow: /"
}

async fn server_kind(State(app): State<App>) -> Response {
  res!(OK, Json(app.server_kind()))
}

async fn validate_token(State(app): State<App>, Json(req): Json<ValidateTokenRequest>) -> Response {
  let player = decode_jwt(&req.token)
    .map(|token| token.claims.sub)
    .ok();

  if app.server_kind().is_remote()
    && let Some(player) = player.clone()
  {
    match app.database().user_exists(player) {
      Ok(true) => {}
      Ok(false) => return res!(OK, Json(None::<PlayerId>)),
      Err(err) => return from_database_err(err),
    }
  }

  res!(OK, Json(player))
}

async fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}

async fn websocket(
  ws: WebSocketUpgrade,
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Query(query): Query<WebsocketQuery>,
) -> Response {
  if app.server_kind().is_remote() {
    match app
      .database()
      .verify_game_password(query.world_id, query.world_password.as_ref())
    {
      Ok(true) => {}
      Ok(false) => return Error::IncorrectWorldCredentials(query.world_id).into(),
      Err(err) => return from_database_err(err),
    }
  }

  let id = player.0;
  app
    .world(query.world_id, World::subscribe)
    .await
    .map_left(|listener| ws.on_upgrade(move |socket| handle_socket(socket, listener, id)))
    .into_inner()
}
