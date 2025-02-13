use crate::state::ServerState;
use crate::{res, response};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use nil_core::{Player, PlayerConfig, PlayerId};

pub async fn get(State(state): State<ServerState>, Json(id): Json<PlayerId>) -> Response {
  state
    .world(|world| world.player(id).cloned())
    .map_ok(|player| res!(OK, Json(player)))
    .unwrap_or_else(response::from_err)
    .await
}

pub async fn get_villages(State(state): State<ServerState>, Json(id): Json<PlayerId>) -> Response {
  state
    .world(|world| world.get_player_villages(id))
    .map(|villages| res!(OK, Json(villages)))
    .await
}

pub async fn spawn(
  Extension(id): Extension<PlayerId>,
  State(state): State<ServerState>,
  Json(config): Json<PlayerConfig>,
) -> Response {
  let player = Player::new(id, config);
  state
    .world_mut(|world| world.spawn_player(player))
    .map_ok(|()| res!(CREATED, Json(id)))
    .unwrap_or_else(response::from_err)
    .await
}
