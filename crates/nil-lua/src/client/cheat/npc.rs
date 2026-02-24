// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::cheat::npc::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("cheatGetEthics", async |lua, this, req: Value| {
    let req: CheatGetEthicsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_get_ethics(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetBotEthics", async |lua, this, req: Value| {
    let req: CheatSetBotEthicsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_bot_ethics(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSpawnBot", async |lua, this, req: Value| {
    let req: CheatSpawnBotRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_spawn_bot(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
