mod player;
mod round;
mod village;

use crate::state::ServerState;
use crate::websocket::handle_socket;
use axum::Router;
use axum::extract::State;
use axum::extract::ws::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{any, get, post, put};
use nil_core::World;

pub(crate) fn create() -> Router<ServerState> {
  Router::new()
    .route("/", get(ok))
    .route("/player", post(player::get))
    .route("/player/spawn", put(player::spawn))
    .route("/player/village", post(player::get_villages))
    .route("/round", get(round::get))
    .route("/version", get(version))
    .route("/village", post(village::get))
    .route("/ws", any(websocket))
}

async fn ok() -> StatusCode {
  StatusCode::OK
}

async fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}

async fn websocket(ws: WebSocketUpgrade, State(state): State<ServerState>) -> Response {
  let listener = state.world(World::subscribe).await;
  ws.on_upgrade(move |socket| handle_socket(socket, listener))
}
