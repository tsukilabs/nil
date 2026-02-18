// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod prefecture;
pub mod stable;
pub mod workshop;

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::infrastructure::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("toggleBuilding", async |lua, this, req: Value| {
    let req: ToggleBuildingRequest = lua.from_value(req)?;
    this
      .client(async |it| it.toggle_building(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
