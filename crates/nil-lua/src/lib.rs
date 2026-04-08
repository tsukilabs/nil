// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]

pub mod client;
pub mod error;
pub mod script;

use client::ClientUserData;
use error::Result;
use mlua::{LuaOptions, StdLib};
use nil_client::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Lua(mlua::Lua);

#[bon::bon]
impl Lua {
  #[builder]
  pub fn new(
    #[builder(start_fn)] client: &Arc<RwLock<Client>>,
    #[builder(default = StdLib::ALL_SAFE)] libs: StdLib,
  ) -> Result<Self> {
    let lua = mlua::Lua::new_with(libs, LuaOptions::default())?;

    let globals = lua.globals();
    let client_data = ClientUserData::new(Arc::clone(client));
    globals.set("client", lua.create_userdata(client_data)?)?;

    Ok(Self(lua))
  }

  pub async fn execute(&mut self, chunk: &str) -> Result<()> {
    self
      .0
      .load(chunk)
      .exec_async()
      .await
      .map_err(Into::into)
  }
}

/// Executes a Lua chunk with the given client.
pub async fn execute(client: &Arc<RwLock<Client>>, chunk: &str) -> Result<()> {
  Lua::builder(client)
    .build()?
    .execute(chunk)
    .await
}
