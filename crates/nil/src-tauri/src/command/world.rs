use crate::error::CResult;
use crate::manager::ManagerExt;
use nil_core::WorldState;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_world_state(app: AppHandle) -> CResult<WorldState> {
  app
    .client(async |cl| cl.world().await)
    .await?
    .map_err(Into::into)
}
