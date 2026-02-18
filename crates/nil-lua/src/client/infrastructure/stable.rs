// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::infrastructure::stable::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("addStableRecruitOrder", async |lua, this, req: Value| {
    let req: AddStableRecruitOrderRequest = lua.from_value(req)?;
    this
      .client(async |it| it.add_stable_recruit_order(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cancelStableRecruitOrder", async |lua, this, req: Value| {
    let req: CancelStableRecruitOrderRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cancel_stable_recruit_order(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getStableRecruitCatalog", async |lua, this, req: Value| {
    let req: GetStableRecruitCatalogRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_stable_recruit_catalog(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
