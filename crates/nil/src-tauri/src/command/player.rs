use crate::error::CResult;
use crate::manager::ManagerExt;
use nil_core::{Coord, Player, PlayerId, PlayerOptions};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_player(app: AppHandle, id: PlayerId) -> CResult<Player> {
  app
    .client(async |it| it.get_player(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_villages(app: AppHandle, id: PlayerId) -> CResult<Vec<Coord>> {
  app
    .client(async |it| it.get_player_villages(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn spawn_player(app: AppHandle, options: PlayerOptions) -> CResult<()> {
  app
    .client(async |it| it.spawn_player(options).await)
    .await?
    .map_err(Into::into)
}
