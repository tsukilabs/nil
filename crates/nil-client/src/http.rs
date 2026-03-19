// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::authorization::Authorization;
use crate::circuit_breaker::{CircuitBreaker, CircuitState};
use crate::error::{Error, Result};
use crate::retry::{Retry, is_retryable_err, is_retryable_status};
use crate::server::ServerAddr;
use anyhow::anyhow;
use futures::TryFutureExt;
use http::{HeaderMap, Method, StatusCode, header};
use reqwest::{Client as HttpClient, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::nonpoison::Mutex;
use std::sync::{LazyLock, Weak};
use tap::{Pipe, Tap, TapFallible};
use tokio::time::{Duration, sleep};

pub const USER_AGENT: &str = concat!("nil/", env!("CARGO_PKG_VERSION"));

static HTTP: LazyLock<HttpClient> = LazyLock::new(|| {
  HttpClient::builder()
    .use_rustls_tls()
    .https_only(false)
    .timeout(Duration::from_mins(1))
    .build()
    .expect("Failed to create HTTP client")
});

#[bon::builder(finish_fn = send)]
pub async fn get(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  authorization: Option<&Authorization>,
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  retry: &Retry,
  user_agent: &str,
) -> Result<()> {
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .circuit_breaker(circuit_breaker)
    .retry(retry)
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
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  retry: &Retry,
  user_agent: &str,
) -> Result<String> {
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .circuit_breaker(circuit_breaker)
    .retry(retry)
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
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  retry: &Retry,
  user_agent: &str,
) -> Result<R>
where
  R: DeserializeOwned,
{
  let url = server.url(route)?;
  request(Method::GET, url.as_str())
    .maybe_authorization(authorization)
    .circuit_breaker(circuit_breaker)
    .retry(retry)
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
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  user_agent: &str,
) -> Result<()> {
  let url = server.url(route)?;
  request_with_body(Method::POST, url.as_str())
    .body(body)
    .maybe_authorization(authorization)
    .circuit_breaker(circuit_breaker)
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
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  user_agent: &str,
) -> Result<R>
where
  R: DeserializeOwned,
{
  let url = server.url(route)?;
  request_with_body(Method::POST, url.as_str())
    .body(body)
    .maybe_authorization(authorization)
    .circuit_breaker(circuit_breaker)
    .user_agent(user_agent)
    .send()
    .and_then(async |res| json::<R>(res).await)
    .await
}

#[bon::builder(finish_fn = send)]
pub async fn json_put<R>(
  #[builder(start_fn)] route: &str,
  #[builder(default)] server: ServerAddr,
  body: impl Serialize,
  authorization: Option<&Authorization>,
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  retry: &Retry,
  user_agent: &str,
) -> Result<R>
where
  R: DeserializeOwned,
{
  let url = server.url(route)?;
  request_with_body(Method::PUT, url.as_str())
    .body(body)
    .maybe_authorization(authorization)
    .circuit_breaker(circuit_breaker)
    .retry(retry)
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
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  retry: Option<&Retry>,
  user_agent: &str,
) -> Result<Response> {
  send_request(method, url)
    .maybe_authorization(authorization)
    .circuit_breaker(circuit_breaker)
    .maybe_retry(retry)
    .user_agent(user_agent)
    .send(async |request| {
      request
        .tap_borrow_dbg(log_request)
        .send()
        .await
        .tap_err_dbg(log_err)?
        .tap_borrow_dbg(log_response)
        .pipe(Ok)
    })
    .await
}

#[bon::builder(finish_fn = send)]
async fn request_with_body(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  body: impl Serialize,
  authorization: Option<&Authorization>,
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  retry: Option<&Retry>,
  user_agent: &str,
) -> Result<Response> {
  send_request(method, url)
    .maybe_authorization(authorization)
    .circuit_breaker(circuit_breaker)
    .maybe_retry(retry)
    .user_agent(user_agent)
    .send(async move |request| {
      request
        .tap_borrow_dbg(log_request)
        .json(&body)
        .send()
        .await
        .tap_err_dbg(log_err)?
        .tap_borrow_dbg(log_response)
        .pipe(Ok)
    })
    .await
}

#[bon::builder(finish_fn = create)]
fn create_request(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  authorization: Option<&Authorization>,
  user_agent: &str,
) -> reqwest::RequestBuilder {
  let mut request = HTTP
    .request(method, url)
    .header(header::USER_AGENT, user_agent);

  if let Some(authorization) = authorization {
    request = request.header(header::AUTHORIZATION, authorization.as_inner());
  }

  request
}

#[bon::builder(finish_fn = send)]
async fn send_request<F>(
  #[builder(start_fn)] method: Method,
  #[builder(start_fn)] url: &str,
  #[builder(finish_fn)] f: F,
  authorization: Option<&Authorization>,
  circuit_breaker: Weak<Mutex<CircuitBreaker>>,
  retry: Option<&Retry>,
  user_agent: &str,
) -> Result<Response>
where
  F: AsyncFn(reqwest::RequestBuilder) -> Result<Response>,
{
  let attempts = retry.map(Retry::attempts).unwrap_or(1);

  for attempt in 1..=attempts {
    if let Some(circuit_breaker) = Weak::upgrade(&circuit_breaker)
      && let CircuitState::Open = circuit_breaker.lock().update()
    {
      return Err(Error::ServiceUnavailable);
    }

    let request = create_request(method.clone(), url)
      .maybe_authorization(authorization)
      .user_agent(user_agent)
      .create();

    let wait_delay = async |retry: &Retry, status: Option<StatusCode>| {
      debug_assert!(
        method == Method::GET || method == Method::PUT,
        "Should only retry idempotent requests"
      );

      let mut delay = retry.delay(attempt);
      if matches!(status, Some(StatusCode::TOO_MANY_REQUESTS)) {
        delay = delay.mul_f64(rand::random_range(1.0..=2.0));
      }

      tracing::warn!(
        %method,
        url,
        attempt,
        max_attempts = attempts,
        retrying_in = ?delay
      );

      sleep(delay).await;
    };

    match f(request).await {
      Ok(response) => {
        let status = response.status();
        if status.is_success() {
          if let Some(circuit_breaker) = Weak::upgrade(&circuit_breaker) {
            circuit_breaker.lock().record_success();
          }

          return Ok(response);
        }

        if attempt < attempts
          && let Some(retry) = retry
          && is_retryable_status(status)
        {
          if let Some(circuit_breaker) = Weak::upgrade(&circuit_breaker) {
            circuit_breaker.lock().record_failure();
          }

          wait_delay(retry, Some(status)).await;
          continue;
        }

        return Err(into_error(response).await?);
      }

      Err(err) => {
        if let Some(circuit_breaker) = Weak::upgrade(&circuit_breaker) {
          circuit_breaker.lock().record_failure();
        }

        if attempt < attempts
          && let Some(retry) = retry
          && is_retryable_err(&err)
        {
          wait_delay(retry, None).await;
          continue;
        }

        return Err(err);
      }
    }
  }

  unreachable!();
}

async fn json<R>(response: Response) -> Result<R>
where
  R: DeserializeOwned,
{
  response
    .json::<R>()
    .await
    .tap_err_dbg(log_err)
    .map_err(Error::Reqwest)
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
    .and_then(|it| it.split(';').next())
    .is_some_and(|it| it.trim().eq_ignore_ascii_case("text/html"))
}

fn log_err(err: &reqwest::Error) {
  tracing::error!(message = %err, error = ?err);
}

fn log_request(request: &reqwest::RequestBuilder) {
  tracing::debug!(?request);
}

fn log_response(response: &Response) {
  let status = response.status();
  tracing::debug!(?status, ?response);
}
