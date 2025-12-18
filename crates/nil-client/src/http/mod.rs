// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod auth;

use crate::error::{Error, Result};
use crate::server::ServerAddr;
use futures::TryFutureExt;
use http::header::AUTHORIZATION;
use http::{HeaderValue, Method};
use nil_core::player::PlayerId;
use reqwest::{Client as HttpClient, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::LazyLock;
use tokio::time::Duration;

pub(crate) use auth::Authorization;

pub const USER_AGENT: &str = concat!("nil/", env!("CARGO_PKG_VERSION"));

static HTTP: LazyLock<HttpClient> = LazyLock::new(|| {
  HttpClient::builder()
    .user_agent(USER_AGENT)
    .use_rustls_tls()
    .timeout(Duration::from_mins(1))
    .build()
    .expect("Failed to create HTTP client")
});

pub struct Http {
  server: ServerAddr,
  authorization: Authorization,
}

impl Http {
  pub(crate) fn new(server: ServerAddr, player: &PlayerId) -> Result<Self> {
    let authorization = player.try_into()?;
    Ok(Self { server, authorization })
  }

  pub async fn get(&self, route: &str) -> Result<()> {
    let url = self.server.url(route)?;
    request(Method::GET, url.as_str())
      .authorization(&self.authorization)
      .call()
      .await
      .map(drop)
  }

  pub async fn get_text(&self, route: &str) -> Result<String> {
    let url = self.server.url(route)?;
    request(Method::GET, url.as_str())
      .authorization(&self.authorization)
      .call()
      .await?
      .text()
      .await
      .map_err(Into::into)
  }

  pub async fn json_get<R>(&self, route: &str) -> Result<R>
  where
    R: DeserializeOwned,
  {
    let url = self.server.url(route)?;
    request(Method::GET, url.as_str())
      .authorization(&self.authorization)
      .call()
      .and_then(async |res| json::<R>(res).await)
      .await
  }

  pub async fn post(&self, route: &str, body: impl Serialize) -> Result<()> {
    let url = self.server.url(route)?;
    request_with_body(Method::POST, url.as_str(), body)
      .authorization(&self.authorization)
      .call()
      .await
      .map(drop)
  }

  pub async fn json_post<R>(&self, route: &str, body: impl Serialize) -> Result<R>
  where
    R: DeserializeOwned,
  {
    let url = self.server.url(route)?;
    request_with_body(Method::POST, url.as_str(), body)
      .authorization(&self.authorization)
      .call()
      .and_then(async |res| json::<R>(res).await)
      .await
  }
}

#[bon::builder]
async fn request(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  authorization: Option<&HeaderValue>,
) -> Result<Response> {
  let mut request = HTTP.request(method, url);
  if let Some(authorization) = authorization {
    request = request.header(AUTHORIZATION, authorization);
  }

  let response = request.send().await?;

  if response.status().is_success() {
    Ok(response)
  } else {
    let text = response.text().await?;
    Err(Error::RequestFailed(text))
  }
}

#[bon::builder]
async fn request_with_body<T>(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  #[builder(start_fn)] body: T,
  authorization: Option<&HeaderValue>,
) -> Result<Response>
where
  T: Serialize,
{
  let mut request = HTTP.request(method, url);
  if let Some(authorization) = authorization {
    request = request.header(AUTHORIZATION, authorization);
  }

  let response = request.json(&body).send().await?;

  if response.status().is_success() {
    Ok(response)
  } else {
    let text = response.text().await?;
    Err(Error::RequestFailed(text))
  }
}

async fn json<R>(response: Response) -> Result<R>
where
  R: DeserializeOwned,
{
  match response.json::<R>().await {
    Ok(value) => Ok(value),
    Err(err) => {
      #[cfg(debug_assertions)]
      tracing::error!(message = %err, error = ?err);

      Err(Error::Reqwest(err))
    }
  }
}
