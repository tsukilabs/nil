pub mod player;
pub mod round;
pub mod village;
pub mod world;

use crate::error::{CResult, Error};
use crate::manager::ManagerExt;
use anyhow::anyhow;
use nil_core::{PlayerId, WorldOptions};
use std::net::SocketAddrV4;
use tauri::{AppHandle, WebviewWindow};

#[tauri::command]
pub async fn get_server_addr(app: AppHandle) -> CResult<SocketAddrV4> {
  app
    .client(async |cl| cl.server_addr())
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_server_version(app: AppHandle) -> CResult<String> {
  app
    .client(async |cl| cl.version().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_dev() -> bool {
  tauri::is_dev()
}

#[tauri::command]
pub async fn is_host(app: AppHandle) -> bool {
  app.nil().is_host().await
}

#[tauri::command]
pub async fn is_mobile() -> bool {
  cfg!(mobile)
}

#[tauri::command]
pub async fn is_server_ready(app: AppHandle) -> CResult<bool> {
  app
    .client(async |cl| cl.ready().await)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  if cfg!(desktop) {
    window
      .show()
      .and_then(|()| window.unminimize())
      .and_then(|()| window.set_focus())
      .map_err(Into::<Error>::into)?;
  }

  Ok(())
}

#[tauri::command]
pub async fn start_client(
  app: AppHandle,
  player_id: PlayerId,
  server_addr: SocketAddrV4,
) -> CResult<()> {
  app
    .nil()
    .start_client(player_id, server_addr)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn start_server(app: AppHandle, world_options: WorldOptions) -> CResult<SocketAddrV4> {
  if cfg!(desktop) {
    app
      .nil()
      .start_server(world_options)
      .await
      .map_err(Into::into)
  } else {
    Err(anyhow!("cannot start server on mobile"))
      .map_err(Into::<Error>::into)
      .map_err(Into::into)
  }
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) -> CResult<()> {
  app
    .nil()
    .stop_client()
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn stop_server(app: AppHandle) {
  app.nil().stop_server().await;
}
