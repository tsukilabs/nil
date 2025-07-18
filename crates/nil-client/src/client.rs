// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod chat;
mod cheat;
mod continent;
mod infrastructure;
mod player;
mod round;
mod script;
mod village;
mod world;

use crate::error::Result;
use crate::http::Http;
use crate::websocket::WebSocketClient;
use futures::future::BoxFuture;
use local_ip_address::local_ip;
use nil_core::event::Event;
use nil_core::player::PlayerId;
use std::net::{IpAddr, SocketAddrV4};

pub struct Client {
  player: PlayerId,
  server: SocketAddrV4,
  http: Http,
  websocket: WebSocketClient,
}

impl Client {
  pub async fn start<F>(player: PlayerId, server: SocketAddrV4, on_event: F) -> Result<Self>
  where
    F: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    let http = Http::new(server, &player)?;
    let websocket = WebSocketClient::connect(&server, &player, on_event).await?;
    Ok(Client { player, server, http, websocket })
  }

  pub async fn stop(self) {
    let _ = self.leave().await;
    self.websocket.stop();
  }

  pub fn player(&self) -> PlayerId {
    self.player.clone()
  }

  pub fn server_addr(&self) -> SocketAddrV4 {
    let mut addr = self.server;
    if addr.ip().is_loopback()
      && let Ok(ip) = local_ip()
      && let IpAddr::V4(ip) = ip
    {
      addr.set_ip(ip);
    }

    addr
  }

  /// GET `/`
  pub async fn is_ready(&self) -> bool {
    self
      .http
      .get("")
      .await
      .map(|()| true)
      .unwrap_or(false)
  }

  /// GET `/leave`
  async fn leave(&self) -> Result<()> {
    self.http.get("leave").await
  }

  /// GET `/version`
  pub async fn version(&self) -> Result<String> {
    self.http.get_text("version").await
  }
}
