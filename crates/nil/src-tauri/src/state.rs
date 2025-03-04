use crate::error::{Error, Result};
use nil_client::Client;
use nil_core::{Event, PlayerId, WorldOptions};
use nil_server::Server;
use std::net::SocketAddrV4;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

pub type NilClient = Arc<RwLock<Option<Client>>>;
pub type NilServer = Arc<RwLock<Option<Server>>>;

pub struct Nil {
  app: AppHandle,
  client: NilClient,
  server: NilServer,
}

impl Nil {
  pub fn new(app: &AppHandle) -> Self {
    Self {
      app: app.clone(),
      client: Arc::new(RwLock::new(None)),
      server: Arc::new(RwLock::new(None)),
    }
  }

  pub async fn client<F, T>(&self, f: F) -> Result<T>
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    let client = &self.client;
    match client.read().await.as_ref() {
      Some(client) => Ok(f(client).await),
      None => Err(Error::ClientClosed),
    }
  }

  pub async fn is_host(&self) -> bool {
    self.server.read().await.is_some()
  }

  pub async fn start_client(&self, player_id: PlayerId, server_addr: SocketAddrV4) -> Result<()> {
    let mut lock = self.client.write().await;
    lock.take();

    let on_event = on_event(self.app.clone());
    let client = Client::start(player_id, server_addr, on_event).await?;
    *lock = Some(client);

    Ok(())
  }

  pub async fn start_server(&self, options: WorldOptions) -> Result<SocketAddrV4> {
    let mut lock = self.server.write().await;
    lock.take();

    let (server, addr) = Server::serve(options).await?;
    *lock = Some(server);

    Ok(addr)
  }

  pub async fn stop_client(&self) -> Result<()> {
    let mut lock = self.client.write().await;
    if let Some(client) = lock.take() {
      let player = client.player_id();
      client.remove_player(player).await?;
    }

    Ok(())
  }

  pub async fn stop_server(&self) {
    self.server.write().await.take();
  }
}

impl Clone for Nil {
  fn clone(&self) -> Self {
    Nil {
      app: self.app.clone(),
      client: Arc::clone(&self.client),
      server: Arc::clone(&self.server),
    }
  }
}

fn on_event(app: AppHandle) -> impl Fn(Event) {
  move |event: Event| {
    let name = event.to_string();
    let _ = app.emit_to("main", &name, event);
  }
}
