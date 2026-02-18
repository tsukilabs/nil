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
mod user;
mod world;

use crate::error::{Error, Result};
use crate::http::authorization::Authorization;
use crate::http::{self, USER_AGENT};
use crate::server::ServerAddr;
use crate::websocket::WebSocketClient;
use futures::future::BoxFuture;
use local_ip_address::local_ip;
use nil_core::event::Event;
use nil_core::player::PlayerId;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use nil_payload::world::LeaveRequest;
use nil_payload::{AuthorizeRequest, ValidateTokenRequest};
use nil_server_types::{ServerKind, Token};
use std::borrow::Cow;
use std::net::{IpAddr, SocketAddrV4};

pub struct Client {
  server: ServerAddr,
  world_id: Option<WorldId>,
  authorization: Option<Authorization>,
  websocket: Option<WebSocketClient>,
  user_agent: Cow<'static, str>,
}

#[bon::bon]
impl Client {
  #[inline]
  pub fn new(server: ServerAddr) -> Self {
    Self {
      server,
      world_id: None,
      authorization: None,
      websocket: None,
      user_agent: Cow::Borrowed(USER_AGENT),
    }
  }

  #[inline]
  pub fn new_local(addr: SocketAddrV4) -> Self {
    Self::new(ServerAddr::Local { addr })
  }

  #[inline]
  pub fn new_remote() -> Self {
    Self::new(ServerAddr::Remote)
  }

  #[builder]
  pub async fn update<OnEvent>(
    &mut self,
    #[builder(start_fn)] server: ServerAddr,
    world_id: Option<WorldId>,
    world_password: Option<Password>,
    player_id: Option<PlayerId>,
    player_password: Option<Password>,
    authorization_token: Option<Token>,
    on_event: Option<OnEvent>,
  ) -> Result<()>
  where
    OnEvent: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    self.stop().await;
    self.server = server;
    self.world_id = world_id;

    if self.server.is_remote()
      && let Some(token) = authorization_token
      && let Some(id) = self.validate_token(&token).await?
      && player_id.as_ref().is_none_or(|it| it == &id)
      && let Ok(authorization) = Authorization::new(token)
    {
      self.authorization = Some(authorization);
    } else if let Some(player) = player_id {
      let req = AuthorizeRequest { player, password: player_password };
      self.authorization = self
        .authorize(req)
        .await
        .map(|token| Some(Authorization::new(&token)))?
        .transpose()
        .inspect_err(|err| tracing::error!(message = %err, error = ?err))
        .map_err(|_| Error::FailedToAuthenticate)?;
    }

    if self.world_id.is_none()
      && self.server.is_local()
      && let ServerKind::Local { id } = self.get_server_kind().await?
    {
      self.world_id = Some(id);
    }

    if let Some(world_id) = self.world_id
      && let Some(on_event) = on_event
      && let Some(authorization) = self.authorization.clone()
    {
      let websocket = WebSocketClient::connect(server)
        .world_id(world_id)
        .maybe_world_password(world_password)
        .authorization(authorization)
        .user_agent(&self.user_agent)
        .on_event(on_event)
        .call()
        .await?;

      self.websocket = Some(websocket);
    }

    Ok(())
  }

  pub async fn stop(&mut self) {
    if let Some(world) = self.world_id
      && self.authorization.is_some()
    {
      let req = LeaveRequest { world };
      if let Err(err) = self.leave(req).await {
        tracing::error!(message = %err, error = ?err);
      }
    }

    self.server = ServerAddr::Remote;
    self.world_id = None;
    self.authorization = None;
    self.websocket = None;
  }

  pub fn server_addr(&self) -> ServerAddr {
    let mut addr = self.server;
    if let ServerAddr::Local { addr } = &mut addr
      && addr.ip().is_loopback()
      && let Ok(ip) = local_ip()
      && let IpAddr::V4(ip) = ip
    {
      addr.set_ip(ip);
    }

    addr
  }

  #[inline]
  pub fn server(&self) -> ServerAddr {
    self.server
  }

  #[inline]
  pub fn world(&self) -> Option<WorldId> {
    self.world_id
  }

  #[inline]
  pub fn user_agent(&self) -> &str {
    &self.user_agent
  }

  pub fn set_user_agent(&mut self, user_agent: &str) {
    self.user_agent = Cow::Owned(user_agent.to_owned());
  }

  #[inline]
  pub fn is_local(&self) -> bool {
    self.server.is_local()
  }

  #[inline]
  pub fn is_remote(&self) -> bool {
    self.server.is_remote()
  }

  pub async fn authorize(&self, req: AuthorizeRequest) -> Result<Token> {
    http::json_post("authorize")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_server_kind(&self) -> Result<ServerKind> {
    http::json_get("get-server-kind")
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_server_version(&self) -> Result<String> {
    http::get_text("version")
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn is_ready(&self) -> bool {
    http::get("")
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
      .map(|()| true)
      .unwrap_or(false)
  }

  pub async fn validate_token<T>(&self, req: T) -> Result<Option<PlayerId>>
  where
    T: Into<ValidateTokenRequest>,
  {
    http::json_post("validate-token")
      .body(Into::<ValidateTokenRequest>::into(req))
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}

impl Default for Client {
  fn default() -> Self {
    Self::new_remote()
  }
}
