// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::report::*;

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getReport", async |lua, this, req: Value| {
    let req: GetReportRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_report(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getReports", async |lua, this, req: Value| {
    let req: GetReportsRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_reports(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
