use super::{Client, USER_AGENT};
use crate::error::{Error, Result};
use http::Method;
use reqwest::{Client as HttpClient, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::LazyLock;
use tokio::time::Duration;

#[cfg(feature = "tracing")]
use tracing::trace;

static HTTP: LazyLock<HttpClient> = LazyLock::new(|| {
  HttpClient::builder()
    .use_rustls_tls()
    .user_agent(USER_AGENT)
    .timeout(Duration::from_secs(10))
    .build()
    .expect("failed to create http client")
});

impl Client {
  async fn request(&self, method: Method, route: &str) -> Result<Response> {
    let url = self.url(route);

    #[cfg(feature = "tracing")]
    trace!("{method}: {url}");

    HTTP
      .request(method, url)
      .send()
      .await
      .and_then(Response::error_for_status)
      .map_err(Error::request_failed)
  }

  async fn request_with_body<T>(&self, method: Method, route: &str, body: T) -> Result<Response>
  where
    T: Serialize + Debug,
  {
    let url = self.url(route);

    #[cfg(feature = "tracing")]
    trace!("{method}: {url}\n{body:?}");

    HTTP
      .request(method, url)
      .json(&body)
      .send()
      .await
      .and_then(Response::error_for_status)
      .map_err(Error::request_failed)
  }

  pub(super) async fn get(&self, route: &str) -> Result<Response> {
    self.request(Method::GET, route).await
  }

  pub(super) async fn get_text(&self, route: &str) -> Result<String> {
    self
      .get(route)
      .await?
      .text()
      .await
      .map_err(Error::failed_to_decode)
  }

  pub(super) async fn post(&self, route: &str, body: impl Serialize + Debug) -> Result<Response> {
    self
      .request_with_body(Method::POST, route, body)
      .await
  }

  pub(super) async fn post_json<R>(&self, route: &str, body: impl Serialize + Debug) -> Result<R>
  where
    R: DeserializeOwned,
  {
    self
      .post(route, body)
      .await?
      .json()
      .await
      .map_err(Error::failed_to_decode)
  }

  pub(super) async fn put(&self, route: &str, body: impl Serialize + Debug) -> Result<Response> {
    self
      .request_with_body(Method::PUT, route, body)
      .await
  }

  pub(super) async fn put_json<R>(&self, route: &str, body: impl Serialize + Debug) -> Result<R>
  where
    R: DeserializeOwned,
  {
    self
      .put(route, body)
      .await?
      .json()
      .await
      .map_err(Error::failed_to_decode)
  }

  fn url(&self, route: &str) -> String {
    let ip = self.server_addr.ip();
    let port = self.server_addr.port();
    format!("http://{ip}:{port}/{route}")
  }
}
