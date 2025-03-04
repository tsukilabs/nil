use crate::state::App;
use crate::{res, response};
use axum::extract::{Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use itertools::Itertools;
use nil_core::player::{Player, PlayerId, PlayerOptions};

pub async fn get(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .player_manager(|pm| pm.player(id).cloned())
    .map_ok(|player| res!(OK, Json(player)))
    .unwrap_or_else(response::from_err)
    .await
}

pub async fn get_all(State(app): State<App>) -> Response {
  app
    .player_manager(|pm| pm.players().cloned().collect_vec())
    .map(|players| res!(OK, Json(players)))
    .await
}

pub async fn remove(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .world_mut(|world| world.remove_player(&id))
    .map_ok(|()| res!(NO_CONTENT))
    .unwrap_or_else(response::from_err)
    .await
}

pub async fn spawn(State(app): State<App>, Json(options): Json<PlayerOptions>) -> Response {
  let player = Player::from(options);
  app
    .world_mut(|world| world.spawn_player(player))
    .map_ok(|()| res!(CREATED))
    .unwrap_or_else(response::from_err)
    .await
}

pub async fn villages(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .continent(|k| k.villages_of(&id))
    .map(|villages| res!(OK, Json(villages)))
    .await
}
