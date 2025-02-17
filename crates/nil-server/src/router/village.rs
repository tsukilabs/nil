use crate::res;
use crate::state::WorldState;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::Coord;

pub async fn get(State(state): State<WorldState>, Json(coord): Json<Coord>) -> Response {
  state
    .world(|world| world.village(coord).cloned())
    .map_ok(|village| res!(OK, Json(village)))
    .unwrap_or_else(|err| res!(NOT_FOUND, err.to_string()))
    .await
}
