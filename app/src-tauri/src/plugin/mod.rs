// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(desktop)]
mod desktop;

use crate::error::BoxResult;
use crate::manager::ManagerExt;
use tap::Pipe;
use tauri::async_runtime::block_on;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{AppHandle, Manager, RunEvent, Wry};
use tauri_plugin_pinia::PrettyTomlMarshaler;

#[cfg(desktop)]
pub use desktop::{prevent_default, single_instance};

pub fn on_exit() -> TauriPlugin<Wry> {
  let task = async |app: &AppHandle| {
    let nil = app.nil();
    nil.stop_client().await;
    nil.stop_server().await;
  };

  Builder::new("on-exit")
    .on_event(move |app, event| {
      if let RunEvent::Exit = event {
        block_on(task(app));
      }
    })
    .build()
}

pub fn pinia(app: &AppHandle) -> BoxResult<TauriPlugin<Wry>> {
  let cache_dir = app.path().app_cache_dir()?;
  tauri_plugin_pinia::Builder::new()
    .path(cache_dir)
    .marshaler(Box::new(PrettyTomlMarshaler))
    .build()
    .pipe(Ok)
}
