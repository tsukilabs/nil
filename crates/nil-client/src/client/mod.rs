// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![expect(clippy::wildcard_imports)]

mod auth;
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

use crate::authorization::Authorization;
use crate::circuit_breaker::CircuitBreaker;
use crate::error::{Error, Result};
use crate::http::{self, USER_AGENT};
use crate::retry::Retry;
use crate::server::ServerAddr;
use crate::websocket::WebSocketClient;
use futures::future::BoxFuture;
use local_ip_address::local_ip;
use nil_core::event::Event;
use nil_core::player::PlayerId;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use nil_payload::request::auth::AuthorizeRequest;
use nil_payload::request::world::LeaveRequest;
use nil_payload::response::server::*;
use nil_server_types::ServerKind;
use nil_server_types::auth::Token;
use std::borrow::Cow;
use std::net::{IpAddr, SocketAddrV4};
use std::sync::nonpoison::Mutex;
use std::sync::{Arc, Weak};

pub struct Client {
  server: ServerAddr,
  world_id: Option<WorldId>,
  authorization: Option<Authorization>,
  websocket: Option<WebSocketClient>,
  circuit_breaker: Arc<Mutex<CircuitBreaker>>,
  retry: Retry,
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
      circuit_breaker: Arc::new(Mutex::default()),
      retry: Retry::with_attempts(2),
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
    #[builder(into)] world_id: Option<WorldId>,
    #[builder(into)] world_password: Option<Password>,
    #[builder(into)] player_id: Option<PlayerId>,
    #[builder(into)] player_password: Option<Password>,
    #[builder(into)] authorization_token: Option<Token>,
    on_event: Option<OnEvent>,
  ) -> Result<()>
  where
    OnEvent: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    let update = Update {
      client: self,
      server,
      world_id,
      world_password,
      player_id,
      player_password,
      authorization_token,
      on_event,
    };

    update.execute().await
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

  fn circuit_breaker(&self) -> Weak<Mutex<CircuitBreaker>> {
    Arc::downgrade(&self.circuit_breaker)
  }

  pub async fn get_server_kind(&self) -> Result<GetServerKindResponse> {
    http::json_get("get-server-kind")
      .server(self.server)
      .retry(&self.retry)
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn is_ready(&self) -> bool {
    http::get("")
      .server(self.server)
      .retry(&self.retry)
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
      .map(|()| true)
      .unwrap_or_else(|err| {
        tracing::error!(message = %err, error = ?err);
        false
      })
  }

  pub async fn version(&self) -> Result<VersionResponse> {
    http::json_get("version")
      .server(self.server)
      .retry(&self.retry)
      .circuit_breaker(self.circuit_breaker())
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

struct Update<'a, OnEvent>
where
  OnEvent: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
{
  client: &'a mut Client,
  server: ServerAddr,
  world_id: Option<WorldId>,
  world_password: Option<Password>,
  player_id: Option<PlayerId>,
  player_password: Option<Password>,
  authorization_token: Option<Token>,
  on_event: Option<OnEvent>,
}

impl<OnEvent> Update<'_, OnEvent>
where
  OnEvent: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
{
  async fn execute(self) -> Result<()> {
    self.client.stop().await;
    self.client.world_id = self.world_id;

    if self.server != self.client.server {
      self.client.server = self.server;
      self
        .client
        .circuit_breaker
        .set(CircuitBreaker::new());
    }

    if self.client.server.is_remote()
      && let Some(token) = self.authorization_token
      && let Some(id) = self.client.validate_token(&token).await?.0
      && self
        .player_id
        .as_ref()
        .is_none_or(|it| it == &id)
      && let Ok(authorization) = Authorization::new(token)
    {
      self.client.authorization = Some(authorization);
    } else if let Some(player) = self.player_id {
      let req = AuthorizeRequest {
        player,
        password: self.player_password,
      };

      self.client.authorization = self
        .client
        .authorize(req)
        .await
        .map(|token| Some(Authorization::new(&token.0)))?
        .transpose()
        .inspect_err(|err| tracing::error!(message = %err, error = ?err))
        .map_err(|_| Error::FailedToAuthenticate)?;
    }

    if self.client.world_id.is_none()
      && self.client.server.is_local()
      && let ServerKind::Local { id } = self.client.get_server_kind().await?.0
    {
      self.client.world_id = Some(id);
    }

    if let Some(world_id) = self.client.world_id
      && let Some(on_event) = self.on_event
      && let Some(authorization) = self.client.authorization.clone()
    {
      let websocket = WebSocketClient::connect(self.client.server)
        .world_id(world_id)
        .maybe_world_password(self.world_password)
        .authorization(authorization)
        .circuit_breaker(self.client.circuit_breaker())
        .user_agent(&self.client.user_agent)
        .on_event(on_event)
        .call()
        .await?;

      self.client.websocket = Some(websocket);
    }

    Ok(())
  }
}
