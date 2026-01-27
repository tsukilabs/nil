// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Result;
use crate::res;
use crate::server::local;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_core::world::{World, WorldId};
use nil_database::model::user_data::UserData;
use nil_database::model::world_data::WorldDataless;
use nil_payload::world::*;

pub async fn get_remote_world(
  State(app): State<App>,
  Json(req): Json<GetRemoteWorldRequest>,
) -> Response {
  if app.server_kind().is_remote() {
    make_remote_world_response(&app, req.world)
      .await
      .map(|world| res!(OK, Json(world)))
      .unwrap_or_else(Response::from)
  } else {
    res!(FORBIDDEN)
  }
}

pub async fn get_remote_worlds(State(app): State<App>) -> Response {
  if app.server_kind().is_remote() {
    let ids = app.world_ids();
    let mut worlds = Vec::with_capacity(ids.len());

    for id in ids {
      if let Ok(world) = make_remote_world_response(&app, id).await {
        worlds.push(world);
      }
    }

    worlds.sort_by_key(|b| std::cmp::Reverse(b.config.id()));

    res!(OK, Json(worlds))
  } else {
    res!(FORBIDDEN)
  }
}

async fn make_remote_world_response(app: &App, id: WorldId) -> Result<GetRemoteWorldResponse> {
  let database = app.database();
  let world_data = WorldDataless::get(&database, id)?;
  let user_data = UserData::get_by_id(&database, world_data.created_by)?;

  let world = app.get(id)?;
  let world = world.read().await;
  let active_players = world
    .players()
    .filter(|player| player.is_active())
    .count();

  Ok(GetRemoteWorldResponse {
    config: world.config().clone(),
    created_by: user_data.user.into(),
    has_password: world_data.password.is_some(),
    active_players,
    current_round: world.round().id(),
  })
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

pub async fn get_stats(State(app): State<App>, Json(req): Json<GetWorldStatsRequest>) -> Response {
  app
    .world(req.world, World::stats)
    .await
    .map_left(|stats| res!(OK, Json(stats)))
    .into_inner()
}

pub async fn save_local(
  State(app): State<App>,
  Json(req): Json<SaveLocalWorldRequest>,
) -> Response {
  if app.server_kind().is_local() {
    let f = move |bytes: Vec<u8>| {
      if let Err(_err) = local::save_local(req.path, &bytes) {
        #[cfg(debug_assertions)]
        tracing::error!(message = %_err, error = ?_err);
      }
    };

    app
      .world_mut(req.world, move |world| world.save(f))
      .await
      .map_left(|()| res!(OK))
      .into_inner()
  } else {
    res!(FORBIDDEN)
  }
}
