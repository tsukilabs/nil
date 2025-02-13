use crate::state::ServerState;
use crate::{res, response};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use nil_core::PlayerId;

pub async fn get(State(state): State<ServerState>) -> Response {
  state
    .world(|world| world.scheduler().turn())
    .map(|turn| res!(OK, Json(turn)))
    .await
}

pub async fn next_player(
  Extension(player): Extension<PlayerId>,
  State(state): State<ServerState>,
) -> Response {
  state
    .world_mut(|world| {
      world.scheduler().assert_turn_of(player)?;
      world.scheduler_mut().next_player()
    })
    .map_ok(|()| res!(OK))
    .unwrap_or_else(response::from_err)
    .await
}
