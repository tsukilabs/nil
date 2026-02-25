// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub(crate) mod authorization;

use crate::error::{Error, Result};
use crate::http::authorization::Authorization;
use crate::server::ServerAddr;
use anyhow::anyhow;
use futures::TryFutureExt;
use http::{HeaderMap, Method, header};
use reqwest::{Client as HttpClient, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::LazyLock;
use tap::Tap;
use tokio::time::Duration;

pub const USER_AGENT: &str = concat!("nil/", env!("CARGO_PKG_VERSION"));

static HTTP: LazyLock<HttpClient> = LazyLock::new(|| {
  HttpClient::builder()
    .https_only(false)
    .timeout(Duration::from_mins(1))
    .tls_backend_rustls()
    .build()
    .expect("Failed to create HTTP client")
});

#[bon::builder(finish_fn = send)]
pub async fn get(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  authorization: Option<&Authorization>,
  user_agent: &str,
) -> Result<()> {
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .user_agent(user_agent)
    .send()
    .await
    .map(drop)
}

#[bon::builder(finish_fn = send)]
pub async fn get_text(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  authorization: Option<&Authorization>,
  user_agent: &str,
) -> Result<String> {
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .user_agent(user_agent)
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
  user_agent: &str,
) -> Result<R>
where
  R: DeserializeOwned,
{
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .user_agent(user_agent)
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
  user_agent: &str,
) -> Result<()> {
  let url = server.url(route)?;
  request_with_body(Method::POST, url.as_str())
    .body(body)
    .maybe_authorization(authorization)
    .user_agent(user_agent)
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
  user_agent: &str,
) -> Result<R>
where
  R: DeserializeOwned,
{
  let url = server.url(route)?;
  request_with_body(Method::POST, url.as_str())
    .body(body)
    .maybe_authorization(authorization)
    .user_agent(user_agent)
    .send()
    .and_then(async |res| json::<R>(res).await)
    .await
}

#[bon::builder(finish_fn = send)]
async fn request(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  authorization: Option<&Authorization>,
  user_agent: &str,
) -> Result<Response> {
  let mut request = HTTP
    .request(method, url)
    .header(header::USER_AGENT, user_agent);

  if let Some(authorization) = authorization {
    request = request.header(header::AUTHORIZATION, authorization.as_inner());
  }

  let response = request
    .tap_borrow_dbg(debug_request)
    .send()
    .await?
    .tap_borrow_dbg(debug_response);

  if response.status().is_success() {
    Ok(response)
  } else {
    Err(into_error(response).await?)
  }
}

#[bon::builder(finish_fn = send)]
async fn request_with_body<T>(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  body: T,
  authorization: Option<&Authorization>,
  user_agent: &str,
) -> Result<Response>
where
  T: Serialize,
{
  let mut request = HTTP
    .request(method, url)
    .header(header::USER_AGENT, user_agent);

  if let Some(authorization) = authorization {
    request = request.header(header::AUTHORIZATION, authorization.as_inner());
  }

  let response = request
    .tap_borrow_dbg(debug_request)
    .json(&body)
    .send()
    .await?
    .tap_borrow_dbg(debug_response);

  if response.status().is_success() {
    Ok(response)
  } else {
    Err(into_error(response).await?)
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

async fn into_error(response: Response) -> Result<Error> {
  let status = response.status();
  let html = has_html_content_type(response.headers());
  let text = response.text().await?;

  if html || text.trim().is_empty() {
    if let Some(reason) = status.canonical_reason() {
      Ok(Error::RequestFailed(reason.to_owned()))
    } else {
      Ok(Error::Unknown(anyhow!("Unknown server error: {status}")))
    }
  } else {
    Ok(Error::RequestFailed(text))
  }
}

pub(crate) fn has_html_content_type(headers: &HeaderMap) -> bool {
  headers
    .get(header::CONTENT_TYPE)
    .and_then(|it| it.to_str().ok())
    .is_some_and(|it| it.eq_ignore_ascii_case("text/html"))
}

fn debug_request(request: &reqwest::RequestBuilder) {
  tracing::debug!(?request);
}

fn debug_response(response: &Response) {
  let status = response.status();
  tracing::debug!(?status, ?response);
}
