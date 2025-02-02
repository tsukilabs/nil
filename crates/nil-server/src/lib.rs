mod error;
mod router;
mod server;
mod websocket;

pub use error::{Error, Result};
use std::net::SocketAddr;
use std::thread;
use tauri::AppHandle;
use tauri::async_runtime::block_on;
use tokio::net::TcpListener;

pub fn serve(app: &AppHandle) -> Result<()> {
  let handle = app.clone();
  let result = app.run_on_main_thread(move || {
    thread::Builder::new()
      .name("nil-server".into())
      .spawn(spawn(handle))
      .expect("failed to spawn nil server");
  });

  result.map_err(Error::failed_to_start)
}

fn spawn(app: AppHandle) -> impl FnOnce() {
  move || {
    block_on(async move {
      let router = router::create()
        .with_state(app)
        .into_make_service_with_connect_info::<SocketAddr>();

      let listener = TcpListener::bind("0.0.0.0:8050")
        .await
        .unwrap();

      axum::serve(listener, router).await.unwrap();
    });
  }
}
