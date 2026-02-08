// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use futures::future::BoxFuture;
use nil_core::event::Event;
use tauri::{AppHandle, Emitter};

pub fn on_core_event(app: AppHandle) -> impl Fn(Event) -> BoxFuture<'static, ()> {
  move |event: Event| {
    let app = app.clone();
    Box::pin(async move {
      let name = format!("nil://{event}");
      if let Err(err) = app.emit_to("main", &name, event) {
        tracing::error!(message = %err, error = ?err);
      }
    })
  }
}
