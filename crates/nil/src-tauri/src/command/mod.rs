pub mod client;
pub mod player;
pub mod server;
pub mod village;

use crate::error::{CResult, Error};
use tauri::WebviewWindow;

#[tauri::command]
pub async fn is_dev() -> bool {
  tauri::is_dev()
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
