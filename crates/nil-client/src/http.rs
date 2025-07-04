// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use futures::TryFutureExt;
use http::header::AUTHORIZATION;
use http::{HeaderValue, Method};
use nil_core::player::PlayerId;
use reqwest::{Client as HttpClient, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::net::SocketAddrV4;
use tokio::time::Duration;

#[cfg(debug_assertions)]
use tracing::error;

pub const USER_AGENT: &str = concat!("nil/", env!("CARGO_PKG_VERSION"));

pub struct Http {
  http: HttpClient,
  server: SocketAddrV4,
  authorization: HeaderValue,
}

impl Http {
  pub fn new(server: SocketAddrV4, player: &PlayerId) -> Result<Self> {
    let Ok(authorization) = HeaderValue::from_str(player) else {
      return Err(Error::InvalidPlayerId(player.clone()));
    };

    let http = HttpClient::builder()
      .use_rustls_tls()
      .user_agent(USER_AGENT)
      .timeout(Duration::from_secs(10))
      .build()?;

    Ok(Self { http, server, authorization })
  }

  async fn request(&self, method: Method, route: &str) -> Result<Response> {
    let response = self
      .http
      .request(method, url(self.server, route))
      .header(AUTHORIZATION, &self.authorization)
      .send()
      .await?;

    if response.status().is_success() {
      Ok(response)
    } else {
      let text = response.text().await?;
      Err(Error::RequestFailed(text))
    }
  }

  async fn request_with_body<T>(&self, method: Method, route: &str, body: T) -> Result<Response>
  where
    T: Serialize,
  {
    let response = self
      .http
      .request(method, url(self.server, route))
      .header(AUTHORIZATION, &self.authorization)
      .json(&body)
      .send()
      .await?;

    if response.status().is_success() {
      Ok(response)
    } else {
      let text = response.text().await?;
      Err(Error::RequestFailed(text))
    }
  }

  pub(crate) async fn get(&self, route: &str) -> Result<()> {
    self
      .request(Method::GET, route)
      .await
      .map(drop)
  }

  pub(crate) async fn get_text(&self, route: &str) -> Result<String> {
    self
      .request(Method::GET, route)
      .await?
      .text()
      .await
      .map_err(Into::into)
  }

  pub(crate) async fn get_json<R>(&self, route: &str) -> Result<R>
  where
    R: DeserializeOwned,
  {
    self
      .request(Method::GET, route)
      .and_then(async |res| json::<R>(res).await)
      .await
  }

  pub(crate) async fn post(&self, route: &str, body: impl Serialize) -> Result<()> {
    self
      .request_with_body(Method::POST, route, body)
      .await
      .map(drop)
  }

  pub(crate) async fn post_json<R>(&self, route: &str, body: impl Serialize) -> Result<R>
  where
    R: DeserializeOwned,
  {
    self
      .request_with_body(Method::POST, route, body)
      .and_then(async |res| json::<R>(res).await)
      .await
  }
}

fn url(server: SocketAddrV4, route: &str) -> String {
  let ip = server.ip();
  let port = server.port();
  format!("http://{ip}:{port}/{route}")
}

async fn json<R>(response: Response) -> Result<R>
where
  R: DeserializeOwned,
{
  match response.json::<R>().await {
    Ok(value) => Ok(value),
    Err(err) => {
      #[cfg(debug_assertions)]
      error!(message = %err, error = ?err);

      Err(Error::Reqwest(err))
    }
  }
}
