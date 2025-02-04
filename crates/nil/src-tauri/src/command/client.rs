use crate::manager::ManagerExt;
use std::net::SocketAddrV4;
use tauri::AppHandle;

#[tauri::command]
pub async fn start_client(app: AppHandle, server_addr: SocketAddrV4) {
  app.nil().start_client(server_addr).await;
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) {
  app.nil().stop_client().await;
}
