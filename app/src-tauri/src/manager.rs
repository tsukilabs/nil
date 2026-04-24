// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::state::Nil;
use nil_client::Client;
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

  fn nil_dir(&self) -> Result<PathBuf> {
    if let Some(home) = env::home_dir() {
      Ok(home.join(".tsukilabs/nil"))
    } else {
      self
        .path()
        .app_local_data_dir()
        .map_err(Into::into)
    }
  }

  fn savedata_dir(&self) -> Result<PathBuf> {
    Ok(self.nil_dir()?.join("savedata"))
  }
}

impl<T: Manager<Wry>> ManagerExt for T {}
