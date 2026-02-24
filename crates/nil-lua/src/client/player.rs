// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::player::*;

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getPlayer", async |lua, this, req: Value| {
    let req: GetPlayerRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_player(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPlayerCoords", async |lua, this, req: Value| {
    let req: GetPlayerCoordsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_player_coords(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPlayerMaintenance", async |lua, this, req: Value| {
    let req: GetPlayerMaintenanceRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_player_maintenance(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPlayerMilitary", async |lua, this, req: Value| {
    let req: GetPlayerMilitaryRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_player_military(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPlayerReports", async |lua, this, req: Value| {
    let req: GetPlayerReportsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_player_reports(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPlayerStatus", async |lua, this, req: Value| {
    let req: GetPlayerStatusRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_player_status(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPlayerStorageCapacity", async |lua, this, req: Value| {
    let req: GetPlayerStorageCapacityRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_player_storage_capacity(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPlayerWorlds", async |lua, this, req: Value| {
    let req: GetPlayerWorldsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_player_worlds(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicPlayer", async |lua, this, req: Value| {
    let req: GetPublicPlayerRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_player(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicPlayers", async |lua, this, req: Value| {
    let req: GetPublicPlayersRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_players(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("playerExists", async |lua, this, req: Value| {
    let req: PlayerExistsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.player_exists(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("setPlayerStatus", async |lua, this, req: Value| {
    let req: SetPlayerStatusRequest = lua.from_value(req)?;
    this
      .client(async |it| it.set_player_status(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("spawnPlayer", async |lua, this, req: Value| {
    let req: SpawnPlayerRequest = lua.from_value(req)?;
    this
      .client(async |it| it.spawn_player(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
