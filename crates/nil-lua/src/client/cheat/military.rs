// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::cheat::military::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("cheatGetIdleArmiesAt", async |lua, this, req: Value| {
    let req: CheatGetIdleArmiesAtRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_get_idle_armies_at(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatGetIdlePersonnelAt", async |lua, this, req: Value| {
    let req: CheatGetIdlePersonnelAtRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_get_idle_personnel_at(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSpawnPersonnel", async |lua, this, req: Value| {
    let req: CheatSpawnPersonnelRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_spawn_personnel(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
