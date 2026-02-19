// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::chat::*;

pub async fn get(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetChatHistoryRequest>,
) -> Response {
  app
    .chat(req.world, |chat| {
      GetChatHistoryResponse {
        public: chat.public(),
        private: chat.private(&player),
      }
    })
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

pub async fn push_stdout(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<PushStdoutMessageRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.push_stdout_message(player.0, &req.stdout);
    })
    .await
    .map_left(|()| res!(OK))
    .into_inner()
}
