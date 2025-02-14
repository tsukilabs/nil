pub mod player;
pub mod round;
pub mod village;

use crate::error::{CResult, Error};
use crate::manager::ManagerExt;
use nil_core::WorldOptions;
use nil_server::ServerInfo;
use std::net::SocketAddrV4;
use tauri::{AppHandle, WebviewWindow};

#[tauri::command]
pub async fn get_server_version(app: AppHandle) -> CResult<String> {
  app
    .client(async |it| it.version().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_dev() -> bool {
  tauri::is_dev()
}

#[tauri::command]
pub async fn is_server_ready(app: AppHandle) -> CResult<bool> {
  app
    .client(async |it| it.ready().await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[cfg(desktop)]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window
    .show()
    .and_then(|()| window.unminimize())
    .and_then(|()| window.set_focus())
    .map_err(Into::<Error>::into)
    .map_err(Into::into)
}

#[tauri::command]
#[cfg(mobile)]
pub async fn show_window() -> CResult<()> {
  Ok(())
}

#[tauri::command]
pub async fn start_client(app: AppHandle, server_addr: SocketAddrV4) -> CResult<()> {
  app
    .nil()
    .start_client(server_addr)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn start_server(app: AppHandle, world_options: WorldOptions) -> CResult<ServerInfo> {
  let world = world_options.into_world();
  app
    .nil()
    .start_server(world)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) {
  app.nil().stop_client().await;
}

#[tauri::command]
pub async fn stop_server(app: AppHandle) {
  app.nil().stop_server().await;
}
