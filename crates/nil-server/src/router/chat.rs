// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::response::EitherExt;
use crate::{bail_if_max_chars_exceeded, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::chat::Chat;
use nil_payload::request::chat::*;
use nil_payload::response::chat::*;

pub async fn get(State(app): State<App>, Json(req): Json<GetChatHistoryRequest>) -> Response {
  app
    .chat(req.world, Chat::history)
    .await
    .map_left(|chat| res!(OK, GetChatHistoryResponse(chat)))
    .into_inner()
}

pub async fn push(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<PushChatMessageRequest>,
) -> Response {
  bail_if_max_chars_exceeded!(req.message, 200);
  app
    .world_mut(req.world, |world| {
      world.push_chat_message(player.0, &req.message)
    })
    .await
    .try_map_left(|id| res!(OK, PushChatMessageResponse(id)))
    .into_inner()
}
