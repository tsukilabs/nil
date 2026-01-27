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

use crate::error::{Error, Result};
use crate::http::Http;
use crate::server::ServerAddr;
use crate::websocket::WebSocketClient;
use futures::future::BoxFuture;
use nil_core::event::Event;
use nil_core::player::PlayerId;
use nil_core::world::WorldId;
use nil_payload::AuthorizeRequest;
use nil_server_types::ServerKind;
use nil_util::password::Password;

pub struct Client {
  http: Http,
  websocket: WebSocketClient,
}

#[bon::bon]
impl Client {
  #[builder]
  pub async fn start<OnEvent>(
    server: ServerAddr,
    mut world_id: Option<WorldId>,
    player_id: PlayerId,
    password: Option<Password>,
    on_event: OnEvent,
  ) -> Result<Self>
  where
    OnEvent: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    let mut http = Http::new(server);
    let authorization = http
      .authorize(AuthorizeRequest { player: player_id, password })
      .await?;

    if world_id.is_none()
      && server.is_local()
      && let ServerKind::Local { id } = http.server_kind().await?
    {
      world_id = Some(id);
    }

    let world_id = world_id.ok_or(Error::MissingWorldId)?;
    let websocket = WebSocketClient::connect(server, world_id, authorization, on_event).await?;

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

  pub async fn get_server_kind(&self) -> Result<ServerKind> {
    self.http.server_kind().await
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
