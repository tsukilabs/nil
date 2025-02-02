use crate::websocket::handle_socket;
use axum::Router;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{any, get};
use std::net::SocketAddr;
use tauri::AppHandle;

pub(crate) fn create() -> Router<AppHandle> {
  Router::new()
    .route("/", get(ok))
    .route("/version", get(version))
    .route("/ws", any(ws_handler))
}

async fn ok() -> StatusCode {
  StatusCode::OK
}

async fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}

async fn ws_handler(
  ws: WebSocketUpgrade,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
  ws.on_upgrade(move |socket| handle_socket(socket, addr))
}
