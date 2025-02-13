mod player;
mod turn;
mod village;

use std::net::SocketAddr;

use crate::state::ServerState;
use crate::websocket::handle_socket;
use axum::Router;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{ConnectInfo, Request, State};
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::{any, get, post, put};
use nil_core::{PlayerId, World};

pub(crate) fn create() -> Router<ServerState> {
  Router::new()
    .route("/", get(ok))
    .route("/player", post(player::get))
    .route("/player/spawn", put(player::spawn))
    .route("/player/villages", post(player::get_villages))
    .route("/turn", get(turn::get))
    .route("/turn/next-player", get(turn::next_player))
    .route("/version", get(version))
    .route("/village", post(village::get))
    .route("/ws", any(ws_handler))
    .layer(middleware::from_fn(with_player_id))
}

async fn with_player_id(
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
  mut request: Request,
  next: Next,
) -> Response {
  let id = PlayerId::from(addr);
  request.extensions_mut().insert(id);
  next.run(request).await
}

async fn ok() -> StatusCode {
  StatusCode::OK
}

async fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<ServerState>) -> Response {
  let listener = state.world(World::subscribe).await;
  ws.on_upgrade(move |socket| handle_socket(socket, listener))
}
