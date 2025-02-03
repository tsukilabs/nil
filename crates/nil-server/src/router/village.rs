use crate::err;
use crate::state::ServerState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use nil_core::Coord;

pub async fn get(State(state): State<ServerState>, Json(coord): Json<Coord>) -> Response {
  match state
    .world(|world| world.village(coord).cloned())
    .await
  {
    Ok(it) => (StatusCode::OK, Json(it)).into_response(),
    Err(e) => err!(NOT_FOUND, e),
  }
}
