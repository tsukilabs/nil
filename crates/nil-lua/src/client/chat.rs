// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::chat::*;

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getChatHistory", async |lua, this, req: Value| {
    let req: GetChatHistoryRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_chat_history(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("pushChatMessage", async |lua, this, req: Value| {
    let req: PushChatMessageRequest = lua.from_value(req)?;
    this
      .client(async |it| it.push_chat_message(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
