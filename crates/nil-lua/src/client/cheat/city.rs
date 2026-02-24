// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::cheat::city::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("cheatSetStability", async |lua, this, req: Value| {
    let req: CheatSetStabilityRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_stability(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
