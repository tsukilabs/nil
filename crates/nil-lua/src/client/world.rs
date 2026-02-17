// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods};

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getRemoteWorlds", async |lua, this, ()| {
    this
      .client(async |it| it.get_remote_worlds().await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
