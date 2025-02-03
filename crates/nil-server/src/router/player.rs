use crate::err;
use crate::state::ServerState;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures::FutureExt;
use nil_core::{Player, PlayerConfig, PlayerId};
use std::net::SocketAddr;

pub async fn get(State(state): State<ServerState>, Json(id): Json<PlayerId>) -> Response {
  match state
    .world(|world| world.player(id).cloned())
    .await
  {
    Ok(player) => (StatusCode::OK, Json(player)).into_response(),
    Err(e) => err!(NOT_FOUND, e),
  }
}

pub async fn get_villages(State(state): State<ServerState>, Json(id): Json<PlayerId>) -> Response {
  state
    .world(|world| world.get_player_villages(id))
    .map(|villages| (StatusCode::OK, Json(villages)))
    .await
    .into_response()
}

pub async fn spawn(
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
  State(state): State<ServerState>,
  Json(config): Json<PlayerConfig>,
) -> Response {
  let id = PlayerId::from(addr);
  let player = Player::new(id, config);
  match state
    .world_mut(|world| world.spawn_player(player))
    .await
  {
    Ok(()) => (StatusCode::CREATED, Json(id)).into_response(),
    Err(e) if e.is_player_already_exists() => err!(CONFLICT, e),
    Err(e) => err!(INTERNAL_SERVER_ERROR, e),
  }
}
