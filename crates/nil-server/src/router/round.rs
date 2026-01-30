// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::{EitherExt, from_core_err, from_database_err};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::world::World;
use nil_payload::round::*;
use tokio::task::spawn_blocking;

pub async fn get(State(app): State<App>, Json(req): Json<GetRoundRequest>) -> Response {
  app
    .round(req.world, Clone::clone)
    .await
    .map_left(|round| res!(OK, Json(round)))
    .into_inner()
}

pub async fn set_ready(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<SetPlayerReadyRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let Ok(result) = spawn_blocking(move || {
        let mut world = world.blocking_write();
        world.set_player_ready(&player, req.is_ready)
      })
      .await
      else {
        return res!(INTERNAL_SERVER_ERROR);
      };

      result
        .map(|()| res!(OK))
        .unwrap_or_else(from_core_err)
    }
    Err(err) => Response::from(err),
  }
}

pub async fn start(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<StartRoundRequest>,
) -> Response {
  if app.server_kind().is_remote() {
    match app
      .database()
      .was_game_created_by(req.world, player)
    {
      Ok(true) => {}
      Ok(false) => return res!(FORBIDDEN),
      Err(err) => return from_database_err(err),
    }
  }

  app
    .world_mut(req.world, World::start_round)
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
