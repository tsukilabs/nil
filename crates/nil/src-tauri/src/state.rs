use crate::error::{Error, Result};
use nil_client::Client;
use nil_core::World;
use nil_server::{Server, ServerInfo};
use std::net::SocketAddrV4;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type NilClient = Arc<RwLock<Option<Client>>>;
pub type NilServer = Arc<RwLock<Option<Server>>>;

pub struct Nil {
  client: NilClient,
  server: NilServer,
}

impl Nil {
  pub fn client(&self) -> NilClient {
    Arc::clone(&self.client)
  }

  pub async fn with_client<F, T>(&self, f: F) -> Result<T>
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    let client = self.client();
    match client.read().await.as_ref() {
      Some(client) => Ok(f(client).await),
      None => Err(Error::ClientClosed),
    }
  }

  pub async fn start_client(&self, server_addr: SocketAddrV4) {
    self
      .client
      .write()
      .await
      .replace(Client::new(server_addr));
  }

  pub async fn start_server(&self, world: World) -> Result<ServerInfo> {
    let mut lock = self.server.write().await;
    lock.take();

    let server = Server::serve(world).await?;
    let server_info = server.info();
    *lock = Some(server);

    Ok(server_info)
  }

  pub async fn stop_client(&self) {
    self.client.write().await.take();
  }

  pub async fn stop_server(&self) {
    self.server.write().await.take();
  }
}

impl Default for Nil {
  fn default() -> Self {
    Self {
      client: Arc::new(RwLock::new(None)),
      server: Arc::new(RwLock::new(None)),
    }
  }
}

impl Clone for Nil {
  fn clone(&self) -> Self {
    Nil {
      client: Arc::clone(&self.client),
      server: Arc::clone(&self.server),
    }
  }
}
