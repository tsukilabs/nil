// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::infrastructure::academy::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("addAcademyRecruitOrder", async |lua, this, req: Value| {
    let req: AddAcademyRecruitOrderRequest = lua.from_value(req)?;
    this
      .client(async |it| it.add_academy_recruit_order(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method(
    "cancelAcademyRecruitOrder",
    async |lua, this, req: Value| {
      let req: CancelAcademyRecruitOrderRequest = lua.from_value(req)?;
      this
        .client(async |it| it.cancel_academy_recruit_order(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method("getAcademyRecruitCatalog", async |lua, this, req: Value| {
    let req: GetAcademyRecruitCatalogRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_academy_recruit_catalog(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
