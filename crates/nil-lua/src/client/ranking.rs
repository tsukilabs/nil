// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::ranking::*;

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getRank", async |lua, this, req: Value| {
    let req: GetRankRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_rank(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getRanking", async |lua, this, req: Value| {
    let req: GetRankingRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_ranking(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
