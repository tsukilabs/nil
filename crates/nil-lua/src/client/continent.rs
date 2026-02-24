// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::continent::*;

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getContinentSize", async |lua, this, req: Value| {
    let req: GetContinentSizeRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_continent_size(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicField", async |lua, this, req: Value| {
    let req: GetPublicFieldRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_field(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicFields", async |lua, this, req: Value| {
    let req: GetPublicFieldsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_fields(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
