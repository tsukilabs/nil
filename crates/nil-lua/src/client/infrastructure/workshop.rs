// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::infrastructure::workshop::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("addWorkshopRecruitOrder", async |lua, this, req: Value| {
    let req: AddWorkshopRecruitOrderRequest = lua.from_value(req)?;
    this
      .client(async |it| it.add_workshop_recruit_order(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method(
    "cancelWorkshopRecruitOrder",
    async |lua, this, req: Value| {
      let req: CancelWorkshopRecruitOrderRequest = lua.from_value(req)?;
      this
        .client(async |it| it.cancel_workshop_recruit_order(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "getWorkshopRecruitCatalog",
    async |lua, this, req: Value| {
      let req: GetWorkshopRecruitCatalogRequest = lua.from_value(req)?;
      this
        .client(async |it| it.get_workshop_recruit_catalog(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );
}
