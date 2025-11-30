// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::state::Nil;
use nil_client::Client;
use tauri::{Manager, State, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn nil(&self) -> State<'_, Nil> {
    self.app_handle().state::<Nil>()
  }

  async fn client<F, T>(&self, f: F) -> Result<T>
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    self.nil().client(f).await
  }
}

impl<T: Manager<Wry>> ManagerExt for T {}
