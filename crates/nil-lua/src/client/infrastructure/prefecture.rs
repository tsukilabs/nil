// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::infrastructure::prefecture::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("addPrefectureBuildOrder", async |lua, this, req: Value| {
    let req: AddPrefectureBuildOrderRequest = lua.from_value(req)?;
    this
      .client(async |it| it.add_prefecture_build_order(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method(
    "cancelPrefectureBuildOrder",
    async |lua, this, req: Value| {
      let req: CancelPrefectureBuildOrderRequest = lua.from_value(req)?;
      this
        .client(async |it| it.cancel_prefecture_build_order(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "getPrefectureBuildCatalog",
    async |lua, this, req: Value| {
      let req: GetPrefectureBuildCatalogRequest = lua.from_value(req)?;
      this
        .client(async |it| it.get_prefecture_build_catalog(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );
}
