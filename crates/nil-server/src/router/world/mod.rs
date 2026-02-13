// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod local;
pub mod remote;

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_core::npc::bot::Bot;
use nil_core::npc::precursor::Precursor;
use nil_core::player::{Player, PlayerStatus};
use nil_core::world::World;
use nil_payload::world::*;

pub async fn get_bots(State(app): State<App>, Json(req): Json<GetWorldBotsRequest>) -> Response {
  app
    .bot_manager(req.world, |bm| bm.bots().map(Bot::id).collect_vec())
    .await
    .map_left(|bots| res!(OK, Json(bots)))
    .into_inner()
}

pub async fn get_config(
  State(app): State<App>,
  Json(req): Json<GetWorldConfigRequest>,
) -> Response {
  app
    .world(req.world, |world| world.config().clone())
    .await
    .map_left(|world| res!(OK, Json(world)))
    .into_inner()
}

pub async fn get_players(
  State(app): State<App>,
  Json(req): Json<GetWorldPlayersRequest>,
) -> Response {
  app
    .player_manager(req.world, |pm| pm.players().map(Player::id).collect_vec())
    .await
    .map_left(|players| res!(OK, Json(players)))
    .into_inner()
}

pub async fn get_precursors(
  State(app): State<App>,
  Json(req): Json<GetWorldPrecursorsRequest>,
) -> Response {
  app
    .precursor_manager(req.world, |pm| {
      pm.precursors()
        .map(Precursor::id)
        .collect_vec()
    })
    .await
    .map_left(|precursors| res!(OK, Json(precursors)))
    .into_inner()
}

pub async fn get_stats(State(app): State<App>, Json(req): Json<GetWorldStatsRequest>) -> Response {
  app
    .world(req.world, World::stats)
    .await
    .map_left(|stats| res!(OK, Json(stats)))
    .into_inner()
}

pub async fn leave(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<LeaveRequest>,
) -> Response {
  if app.server_kind().is_local() {
    app
      .world_mut(req.world, |world| {
        world.set_player_status(&player.0, PlayerStatus::Inactive)
      })
      .await
      .try_map_left(|()| res!(OK))
      .into_inner()
  } else {
    res!(OK)
  }
}
