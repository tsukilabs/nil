// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod battle;
mod chat;
mod cheat;
mod city;
mod continent;
mod infrastructure;
mod military;
mod npc;
mod player;
mod ranking;
mod report;
mod round;
mod user;
mod world;

use mlua::{LuaSerdeExt, UserData, UserDataMethods};
use nil_client::Client;
use serde::Serialize;
use std::sync::Arc;
use tap::Pipe;
use tokio::sync::RwLock;

pub struct ClientUserData {
  client: Arc<RwLock<Client>>,
}

impl ClientUserData {
  pub fn new(client: Arc<RwLock<Client>>) -> Self {
    Self { client }
  }

  async fn client<F, T>(&self, f: F) -> T
  where
    F: AsyncFnOnce(&Client) -> T,
    T: Serialize,
  {
    f(&*self.client.read().await).await
  }
}

impl UserData for ClientUserData {
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    city::add_methods(methods);
    world::add_methods(methods);

    methods.add_async_method("getServerKind", async |lua, this, ()| {
      this
        .client(async |it| it.get_server_kind().await)
        .await
        .map(|it| lua.to_value(&it))?
    });

    methods.add_async_method("getServerVersion", async |lua, this, ()| {
      this
        .client(async |it| it.get_server_version().await)
        .await
        .map(|it| lua.to_value(&it))?
    });

    methods.add_async_method("isServerLocal", async |lua, this, ()| {
      this
        .client(async |it| it.is_local())
        .await
        .pipe(|it| lua.to_value(&it))
    });

    methods.add_async_method("isServerReady", async |lua, this, ()| {
      this
        .client(async |it| it.is_ready().await)
        .await
        .pipe(|it| lua.to_value(&it))
    });

    methods.add_async_method("isServerRemote", async |lua, this, ()| {
      this
        .client(async |it| it.is_remote())
        .await
        .pipe(|it| lua.to_value(&it))
    });

    methods.add_async_method("server", async |lua, this, ()| {
      this
        .client(async |it| it.server())
        .await
        .pipe(|it| lua.to_value(&it))
    });

    methods.add_async_method("userAgent", async |lua, this, ()| {
      this
        .client(async |it| it.user_agent().to_owned())
        .await
        .pipe(|it| lua.to_value(&it))
    });

    methods.add_method("version", |lua, _, ()| lua.to_value(nil_client::VERSION));

    methods.add_async_method("world", async |lua, this, ()| {
      this
        .client(async |it| it.world())
        .await
        .pipe(|it| lua.to_value(&it))
    });
  }
}
