// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::cheat::resources::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("cheatGetResources", async |lua, this, req: Value| {
    let req: CheatGetResourcesRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_get_resources(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetFood", async |lua, this, req: Value| {
    let req: CheatSetFoodRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_food(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetIron", async |lua, this, req: Value| {
    let req: CheatSetIronRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_iron(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetMaxFood", async |lua, this, req: Value| {
    let req: CheatSetMaxFoodRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_max_food(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetMaxIron", async |lua, this, req: Value| {
    let req: CheatSetMaxIronRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_max_iron(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetMaxResources", async |lua, this, req: Value| {
    let req: CheatSetMaxResourcesRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_max_resources(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetMaxSiloResources", async |lua, this, req: Value| {
    let req: CheatSetMaxSiloResourcesRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_max_silo_resources(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetMaxStone", async |lua, this, req: Value| {
    let req: CheatSetMaxStoneRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_max_stone(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method(
    "cheatSetMaxWarehouseResources",
    async |lua, this, req: Value| {
      let req: CheatSetMaxWarehouseResourcesRequest = lua.from_value(req)?;
      this
        .client(async |it| {
          it.cheat_set_max_warehouse_resources(req)
            .await
        })
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method("cheatSetMaxWood", async |lua, this, req: Value| {
    let req: CheatSetMaxWoodRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_max_wood(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetResources", async |lua, this, req: Value| {
    let req: CheatSetResourcesRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_resources(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetStone", async |lua, this, req: Value| {
    let req: CheatSetStoneRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_stone(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetWood", async |lua, this, req: Value| {
    let req: CheatSetWoodRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_wood(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
