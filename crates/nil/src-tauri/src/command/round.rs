use crate::error::CResult;
use crate::manager::ManagerExt;
use nil_core::RoundState;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_round_state(app: AppHandle) -> CResult<RoundState> {
  app
    .client(async |it| it.get_round_state().await)
    .await?
    .map_err(Into::into)
}
