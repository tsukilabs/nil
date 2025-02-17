use crate::state::WorldState;
use crate::{res, response};
use axum::extract::{Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use nil_core::{Player, PlayerId, PlayerOptions};

pub async fn get(State(state): State<WorldState>, Json(id): Json<PlayerId>) -> Response {
  state
    .world(|world| world.player(id).cloned())
    .map_ok(|player| res!(OK, Json(player)))
    .unwrap_or_else(response::from_err)
    .await
}

pub async fn get_villages(State(state): State<WorldState>, Json(id): Json<PlayerId>) -> Response {
  state
    .world(|world| world.villages_of(&id))
    .map(|villages| res!(OK, Json(villages)))
    .await
}

pub async fn spawn(
  State(state): State<WorldState>,
  Json(options): Json<PlayerOptions>,
) -> Response {
  let player = Player::from(options);
  state
    .world_mut(|world| world.spawn_player(player))
    .map_ok(|()| res!(CREATED))
    .unwrap_or_else(response::from_err)
    .await
}
