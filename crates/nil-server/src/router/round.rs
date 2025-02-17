use crate::res;
use crate::state::WorldState;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;

pub async fn get(State(state): State<WorldState>) -> Response {
  state
    .world(|world| world.round_state())
    .map(|round| res!(OK, Json(round)))
    .await
}
