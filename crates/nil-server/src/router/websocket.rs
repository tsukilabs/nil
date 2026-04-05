// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Error;
use crate::middleware::authorization::CurrentPlayer;
use crate::response::from_err;
use crate::websocket::handle_socket;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Extension, Query, State};
use axum::response::Response;
use nil_core::world::World;
use nil_payload::WebsocketQuery;

pub async fn websocket(
  ws: WebSocketUpgrade,
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Query(query): Query<WebsocketQuery>,
) -> Response {
  if app.server_kind().is_remote() {
    match app
      .database()
      .verify_game_password(query.world_id, query.world_password.as_ref())
    {
      Ok(true) => {}
      Ok(false) => return from_err(Error::IncorrectWorldCredentials(query.world_id)),
      Err(err) => return from_err(err),
    }
  }

  let id = player.0;
  app
    .world(query.world_id, World::subscribe)
    .await
    .map_left(|listener| ws.on_upgrade(move |socket| handle_socket(socket, listener, id)))
    .into_inner()
}
