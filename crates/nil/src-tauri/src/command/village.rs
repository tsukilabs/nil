use crate::error::CResult;
use crate::manager::ManagerExt;
use nil_core::{Coord, Village};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_village(app: AppHandle, coord: Coord) -> CResult<Village> {
  app
    .client(async |cl| cl.village(coord).await)
    .await?
    .map_err(Into::into)
}
