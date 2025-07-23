// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::FutureExt;

pub async fn get(State(app): State<App>) -> Response {
  app
    .chat(Clone::clone)
    .map(|chat| res!(OK, Json(chat)))
    .await
}

pub async fn push(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(message): Json<String>,
) -> Response {
  app
    .world_mut(|world| world.push_chat_message(player.0, &message))
    .map(|id| res!(OK, Json(id)))
    .await
}
