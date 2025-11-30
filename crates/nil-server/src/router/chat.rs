// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::FutureExt;
use nil_payload::chat::PushChatMessageRequest;

pub async fn get(State(app): State<App>, Extension(player): Extension<CurrentPlayer>) -> Response {
  app
    .chat(|chat| (chat.public(), chat.private(&player)))
    .map(|chat| res!(OK, Json(chat)))
    .await
}

pub async fn push(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<PushChatMessageRequest>,
) -> Response {
  app
    .world_mut(|world| world.push_chat_message(player.0, &req.message))
    .map(|()| res!(OK))
    .await
}
