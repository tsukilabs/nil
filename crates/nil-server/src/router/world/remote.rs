// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Result;
use crate::middleware::authorization::CurrentPlayer;
use crate::{VERSION, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::world::config::WorldId;
use nil_payload::world::*;
use nil_server_types::RemoteWorld;
use semver::Version;
use tokio::task::spawn_blocking;

pub async fn create(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CreateRemoteWorldRequest>,
) -> Response {
  if app.server_kind().is_remote() {
    let Ok(version) = Version::parse(VERSION) else {
      return res!(INTERNAL_SERVER_ERROR);
    };

    let Ok(result) = spawn_blocking(move || {
      app
        .create_remote(&req.options)
        .player_id(player)
        .maybe_world_description(req.description)
        .maybe_world_password(req.password.as_ref())
        .server_version(version)
        .call()
    })
    .await
    else {
      return res!(INTERNAL_SERVER_ERROR);
    };

    result
      .map(|world_id| res!(CREATED, Json(world_id)))
      .unwrap_or_else(Response::from)
  } else {
    res!(FORBIDDEN)
  }
}

pub async fn get(State(app): State<App>, Json(req): Json<GetRemoteWorldRequest>) -> Response {
  if app.server_kind().is_remote() {
    let Ok(result) = spawn_blocking(move || make_remote_world(&app, req.world)).await else {
      return res!(INTERNAL_SERVER_ERROR);
    };

    result
      .map(|world| res!(OK, Json(world)))
      .unwrap_or_else(Response::from)
  } else {
    res!(FORBIDDEN)
  }
}

pub async fn get_all(State(app): State<App>) -> Response {
  if app.server_kind().is_remote() {
    let Ok(worlds) = spawn_blocking(move || {
      let ids = app.world_ids();
      let mut worlds = Vec::with_capacity(ids.len());

      for id in ids {
        if let Ok(world) = make_remote_world(&app, id) {
          worlds.push(world);
        }
      }

      worlds.sort_by_key(|b| std::cmp::Reverse(b.config.id()));
      worlds
    })
    .await
    else {
      return res!(INTERNAL_SERVER_ERROR);
    };

    res!(OK, Json(worlds))
  } else {
    res!(FORBIDDEN)
  }
}

fn make_remote_world(app: &App, id: WorldId) -> Result<RemoteWorld> {
  let database = app.database();
  let game = database.get_game(id)?;
  let user = database.get_user_by_id(game.created_by)?;

  let world = app.get(id)?;
  let world = world.blocking_read();

  let mut active_players: usize = 0;
  let mut total_players: usize = 0;

  for player in world.players() {
    if player.is_active() {
      active_players = active_players.saturating_add(1);
    }

    total_players = total_players.saturating_add(1);
  }

  Ok(RemoteWorld {
    config: world.config().clone(),
    description: game.description,
    created_by: user.player_id.into(),
    created_at: game.created_at.into(),
    updated_at: game.updated_at.into(),
    has_password: game.password.is_some(),
    current_round: world.round().id(),
    active_players,
    total_players,
    continent_size: world.continent().size(),
  })
}
