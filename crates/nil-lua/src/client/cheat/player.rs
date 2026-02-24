// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::cheat::player::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("cheatGetPlayer", async |lua, this, req: Value| {
    let req: CheatGetPlayerRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_get_player(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatGetPlayers", async |lua, this, req: Value| {
    let req: CheatGetPlayersRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_get_players(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
