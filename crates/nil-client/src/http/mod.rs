// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub(crate) mod authorization;

use crate::error::{Error, Result};
use crate::http::authorization::Authorization;
use crate::server::ServerAddr;
use futures::TryFutureExt;
use http::Method;
use http::header::AUTHORIZATION;
use reqwest::{Client as HttpClient, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::LazyLock;
use tokio::time::Duration;

pub const USER_AGENT: &str = concat!("nil/", env!("CARGO_PKG_VERSION"));

static HTTP: LazyLock<HttpClient> = LazyLock::new(|| {
  HttpClient::builder()
    .user_agent(USER_AGENT)
    .use_rustls_tls()
    .timeout(Duration::from_mins(1))
    .build()
    .expect("Failed to create HTTP client")
});

#[bon::builder(finish_fn = send)]
pub async fn get(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  authorization: Option<&Authorization>,
) -> Result<()> {
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .send()
    .await
    .map(drop)
}

#[bon::builder(finish_fn = send)]
pub async fn get_text(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  authorization: Option<&Authorization>,
) -> Result<String> {
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .send()
    .await?
    .text()
    .await
    .map_err(Into::into)
}

#[bon::builder(finish_fn = send)]
pub async fn json_get<R>(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  authorization: Option<&Authorization>,
) -> Result<R>
where
  R: DeserializeOwned,
{
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .send()
    .and_then(async |res| json::<R>(res).await)
    .await
}

#[bon::builder(finish_fn = send)]
pub async fn post(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  body: impl Serialize,
  authorization: Option<&Authorization>,
) -> Result<()> {
  let url = server.url(route)?;
  request_with_body(Method::POST, url.as_str())
    .body(body)
    .maybe_authorization(authorization)
    .send()
    .await
    .map(drop)
}

#[bon::builder(finish_fn = send)]
pub async fn json_post<R>(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  body: impl Serialize,
  authorization: Option<&Authorization>,
) -> Result<R>
where
  R: DeserializeOwned,
{
  let url = server.url(route)?;
  request_with_body(Method::POST, url.as_str())
    .body(body)
    .maybe_authorization(authorization)
    .send()
    .and_then(async |res| json::<R>(res).await)
    .await
}

#[bon::builder(finish_fn = send)]
async fn request(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  authorization: Option<&Authorization>,
) -> Result<Response> {
  let mut request = HTTP.request(method, url);
  if let Some(authorization) = authorization {
    request = request.header(AUTHORIZATION, authorization.as_inner());
  }

  let response = request.send().await?;

  if response.status().is_success() {
    Ok(response)
  } else {
    let text = response.text().await?;
    Err(Error::RequestFailed(text))
  }
}

#[bon::builder(finish_fn = send)]
async fn request_with_body<T>(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  body: T,
  authorization: Option<&Authorization>,
) -> Result<Response>
where
  T: Serialize,
{
  let mut request = HTTP.request(method, url);
  if let Some(authorization) = authorization {
    request = request.header(AUTHORIZATION, authorization.as_inner());
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
      tracing::error!(message = %err, error = ?err);
      Err(Error::Reqwest(err))
    }
  }
}
