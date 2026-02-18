// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::city::*;

pub(super) fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method("getCity", async |lua, this, req: Value| {
    let req: GetCityRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_city(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getCityScore", async |lua, this, req: Value| {
    let req: GetCityScoreRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_city_score(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicCities", async |lua, this, req: Value| {
    let req: GetPublicCitiesRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_cities(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("getPublicCity", async |lua, this, req: Value| {
    let req: GetPublicCityRequest = lua.from_value(req)?;
    this
      .client(async |it| it.get_public_city(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("renameCity", async |lua, this, req: Value| {
    let req: RenameCityRequest = lua.from_value(req)?;
    this
      .client(async |it| it.rename_city(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("searchCity", async |lua, this, req: Value| {
    let req: SearchCityRequest = lua.from_value(req)?;
    this
      .client(async |it| it.search_city(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("searchPublicCity", async |lua, this, req: Value| {
    let req: SearchPublicCityRequest = lua.from_value(req)?;
    this
      .client(async |it| it.search_public_city(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });
}
