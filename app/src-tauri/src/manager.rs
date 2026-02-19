// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::state::Nil;
use nil_client::Client;
use nil_lua::Lua;
use std::env;
use std::path::PathBuf;
use tauri::{Manager, State, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn nil(&self) -> State<'_, Nil> {
    self.app_handle().state::<Nil>()
  }

  async fn client<F, T>(&self, f: F) -> T
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    self.nil().client(f).await
  }

  async fn lua<F, T>(&self, f: F) -> T
  where
    F: AsyncFnOnce(&mut Lua) -> T,
  {
    self.nil().lua(f).await
  }

  fn nil_dir(&self) -> Result<PathBuf> {
    let mut dir = if let Some(home) = env::home_dir() {
      home.join(env!("NIL_DIR"))
    } else {
      self.path().app_cache_dir()?
    };

    if cfg!(debug_assertions) {
      dir.push(".dev");
    }

    Ok(dir)
  }

  fn savedata_dir(&self) -> Result<PathBuf> {
    Ok(self.nil_dir()?.join("savedata"))
  }

  fn script_dir(&self) -> Result<PathBuf> {
    Ok(self.nil_dir()?.join("scripts"))
  }
}

impl<T: Manager<Wry>> ManagerExt for T {}
