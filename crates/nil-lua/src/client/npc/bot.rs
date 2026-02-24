// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::npc::bot::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getBotCoords", async |lua, this, req: Value| {
    let req: GetBotCoordsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_bot_coords(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicBot", async |lua, this, req: Value| {
    let req: GetPublicBotRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_bot(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicBots", async |lua, this, req: Value| {
    let req: GetPublicBotsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_bots(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
