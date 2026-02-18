// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::world::*;

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("createRemoteWorld", async |lua, this, req: Value| {
    let req: CreateRemoteWorldRequest = lua.from_value(req)?;
    this
      .client(async |it| it.create_remote_world(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getRemoteWorld", async |lua, this, req: Value| {
    let req: GetRemoteWorldRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_remote_world(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getRemoteWorlds", async |lua, this, ()| {
    this
      .client(async |it| it.get_remote_worlds().await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getWorldBots", async |lua, this, req: Value| {
    let req: GetWorldBotsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_world_bots(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getWorldConfig", async |lua, this, req: Value| {
    let req: GetWorldConfigRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_world_config(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getWorldPlayers", async |lua, this, req: Value| {
    let req: GetWorldPlayersRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_world_players(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getWorldPrecursors", async |lua, this, req: Value| {
    let req: GetWorldPrecursorsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_world_precursors(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getWorldStats", async |lua, this, req: Value| {
    let req: GetWorldStatsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_world_stats(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("saveLocalWorld", async |lua, this, req: Value| {
    let req: SaveLocalWorldRequest = lua.from_value(req)?;
    this
      .client(async |it| it.save_local_world(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
