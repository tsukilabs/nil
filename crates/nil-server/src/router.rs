mod player;
mod round;
mod village;
mod world;

use crate::state::App;
use crate::websocket::handle_socket;
use axum::Router;
use axum::extract::State;
use axum::extract::ws::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{any, get, post};
use nil_core::World;

pub(crate) fn create() -> Router<App> {
  Router::new()
    .route("/", get(ok))
    .route("/player", get(player::get_all))
    .route("/player", post(player::get))
    .route("/player/remove", post(player::remove))
    .route("/player/spawn", post(player::spawn))
    .route("/player/village", post(player::villages))
    .route("/round", get(round::get))
    .route("/version", get(version))
    .route("/village", post(village::get))
    .route("/world", get(world::get))
    .route("/ws", any(websocket))
}

async fn ok() -> StatusCode {
  StatusCode::OK
}

async fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}

async fn websocket(ws: WebSocketUpgrade, State(app): State<App>) -> Response {
  let listener = app.world(World::subscribe).await;
  ws.on_upgrade(move |socket| handle_socket(socket, listener))
}
