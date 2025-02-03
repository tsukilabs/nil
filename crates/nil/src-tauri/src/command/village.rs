use crate::error::CResult;
use crate::manager::ManagerExt;
use nil_core::{Coord, Village};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_village(app: AppHandle, coord: Coord) -> CResult<Village> {
  app
    .with_client(async |client| client.get_village(coord).await)
    .await?
    .map_err(Into::into)
}
