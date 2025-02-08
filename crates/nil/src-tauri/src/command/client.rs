use crate::error::CResult;
use crate::manager::ManagerExt;
use std::net::SocketAddrV4;
use tauri::AppHandle;

#[tauri::command]
pub async fn start_client(app: AppHandle, server_addr: SocketAddrV4) -> CResult<()> {
  app
    .nil()
    .start_client(server_addr)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) {
  app.nil().stop_client().await;
}
