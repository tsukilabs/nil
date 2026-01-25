// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![expect(clippy::wildcard_imports)]

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
mod world;

use crate::error::Result;
use crate::http::Http;
use crate::server::ServerAddr;
use crate::websocket::WebSocketClient;
use futures::future::BoxFuture;
use nil_core::event::Event;
use nil_core::player::PlayerId;

pub struct Client {
  http: Http,
  websocket: WebSocketClient,
}

impl Client {
  pub async fn start<F>(server: ServerAddr, player: PlayerId, on_event: F) -> Result<Self>
  where
    F: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    let http = Http::new(server, &player)?;
    let websocket = WebSocketClient::connect(server, &player, on_event).await?;
    Ok(Client { http, websocket })
  }

  pub async fn stop(self) {
    let _ = self.leave().await;
    self.websocket.stop();
  }

  #[inline]
  pub fn http(&self) -> &Http {
    &self.http
  }

  #[inline]
  pub fn server_addr(&self) -> ServerAddr {
    self.http.server_addr()
  }

  pub async fn is_ready(&self) -> bool {
    self
      .http
      .get("")
      .await
      .map(|()| true)
      .unwrap_or(false)
  }

  async fn leave(&self) -> Result<()> {
    self.http.get("leave").await
  }

  pub async fn version(&self) -> Result<String> {
    self.http.get_text("version").await
  }
}
