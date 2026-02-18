// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::round::*;

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getRound", async |lua, this, req: Value| {
    let req: GetRoundRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_round(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("setPlayerReady", async |lua, this, req: Value| {
    let req: SetPlayerReadyRequest = lua.from_value(req)?;
    this
      .client(async |it| it.set_player_ready(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("startRound", async |lua, this, req: Value| {
    let req: StartRoundRequest = lua.from_value(req)?;
    this
      .client(async |it| it.start_round(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
