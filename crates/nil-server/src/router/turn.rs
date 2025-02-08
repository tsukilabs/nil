use crate::state::ServerState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures::FutureExt;

pub async fn get(State(state): State<ServerState>) -> Response {
  state
    .world(|world| world.turn_scheduler().turn())
    .map(|turn| (StatusCode::OK, Json(turn)))
    .await
    .into_response()
}
