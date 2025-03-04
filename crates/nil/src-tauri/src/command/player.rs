use crate::error::CResult;
use crate::manager::ManagerExt;
use nil_core::{Coord, Player, PlayerId, PlayerOptions};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_player(app: AppHandle, id: PlayerId) -> CResult<Player> {
  app
    .client(async |cl| cl.player(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_villages(app: AppHandle, id: PlayerId) -> CResult<Vec<Coord>> {
  app
    .client(async |cl| cl.villages_of(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_players(app: AppHandle) -> CResult<Vec<Player>> {
  app
    .client(async |cl| cl.players().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn remove_player(app: AppHandle, id: PlayerId) -> CResult<()> {
  app
    .client(async |cl| cl.remove_player(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn spawn_player(app: AppHandle, options: PlayerOptions) -> CResult<()> {
  app
    .client(async |cl| cl.spawn_player(options).await)
    .await?
    .map_err(Into::into)
}
