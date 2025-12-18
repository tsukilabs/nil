// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::chat::*;

pub async fn get(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetChatHistoryRequest>,
) -> Response {
  app
    .chat(req.world, |chat| (chat.public(), chat.private(&player)))
    .await
    .map_left(|chat| res!(OK, Json(chat)))
    .into_inner()
}

pub async fn push(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<PushChatMessageRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.push_chat_message(player.0, &req.message);
    })
    .await
    .map_left(|()| res!(OK))
    .into_inner()
}
