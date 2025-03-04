use super::{Client, USER_AGENT};
use crate::error::{Error, Result};
use http::Method;
use reqwest::{Client as HttpClient, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::LazyLock;
use tokio::time::Duration;

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
    HTTP
      .request(method, self.url(route))
      .send()
      .await
      .and_then(Response::error_for_status)
      .map_err(Error::request_failed)
  }

  async fn request_with_body<T>(&self, method: Method, route: &str, body: T) -> Result<Response>
  where
    T: Serialize + Debug,
  {
    HTTP
      .request(method, self.url(route))
      .json(&body)
      .send()
      .await
      .and_then(Response::error_for_status)
      .map_err(Error::request_failed)
  }

  pub(super) async fn get(&self, route: &str) -> Result<()> {
    self
      .request(Method::GET, route)
      .await
      .map(drop)
  }

  pub(super) async fn get_text(&self, route: &str) -> Result<String> {
    self
      .request(Method::GET, route)
      .await?
      .text()
      .await
      .map_err(Error::failed_to_decode)
  }

  pub(super) async fn get_json<R>(&self, route: &str) -> Result<R>
  where
    R: DeserializeOwned,
  {
    self
      .request(Method::GET, route)
      .await?
      .json()
      .await
      .map_err(Error::failed_to_decode)
  }

  pub(super) async fn post(&self, route: &str, body: impl Serialize + Debug) -> Result<()> {
    self
      .request_with_body(Method::POST, route, body)
      .await
      .map(drop)
  }

  pub(super) async fn post_json<R>(&self, route: &str, body: impl Serialize + Debug) -> Result<R>
  where
    R: DeserializeOwned,
  {
    self
      .request_with_body(Method::POST, route, body)
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
