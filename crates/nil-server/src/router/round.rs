use crate::res;
use crate::state::ServerState;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;

pub async fn get(State(state): State<ServerState>) -> Response {
  state
    .world(|world| world.round().state())
    .map(|turn| res!(OK, Json(turn)))
    .await
}
