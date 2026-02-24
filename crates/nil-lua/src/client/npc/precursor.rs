// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::npc::precursor::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getPrecursorCoords", async |lua, this, req: Value| {
    let req: GetPrecursorCoordsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_precursor_coords(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicPrecursor", async |lua, this, req: Value| {
    let req: GetPublicPrecursorRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_precursor(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicPrecursors", async |lua, this, req: Value| {
    let req: GetPublicPrecursorsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_precursors(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
