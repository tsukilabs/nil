// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::queue::push_event;
use futures::future::BoxFuture;
use nil_client::Client;
use nil_core::event::Event;
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub(crate) static CLIENT: LazyLock<RwLock<Client>> = LazyLock::new(RwLock::default);

pub(crate) fn on_event() -> impl Fn(Event) -> BoxFuture<'static, ()> {
  move |event: Event| Box::pin(async move { push_event(event) })
}
