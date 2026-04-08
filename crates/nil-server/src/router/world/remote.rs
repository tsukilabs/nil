// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Result;
use crate::middleware::authorization::CurrentPlayer;
use crate::response::from_err;
use crate::{VERSION, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::world::config::WorldId;
use nil_payload::world::*;
use nil_server_types::world::RemoteWorld;
use semver::Version;
use std::cmp::Reverse;
use std::sync::Arc;

pub async fn create(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CreateRemoteWorldRequest>,
) -> Response {
  if app.server_kind().is_remote() {
    let Ok(version) = Version::parse(VERSION) else {
      return res!(INTERNAL_SERVER_ERROR);
    };

    app
      .create_remote(&req.options)
      .player_id(player)
      .maybe_world_description(req.description)
      .maybe_world_password(req.password)
      .maybe_round_duration(req.round_duration)
      .server_version(version)
      .call()
      .await
      .map(|world_id| res!(CREATED, Json(world_id)))
      .unwrap_or_else(from_err)
  } else {
    res!(FORBIDDEN)
  }
}

pub async fn delete(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<DeleteRemoteWorldRequest>,
) -> Response {
  if app.server_kind().is_remote() {
    let result = try {
      let database = app.database();
      if database
        .was_game_created_by(req.world, player.0)
        .await?
      {
        database.delete_game(req.world).await?;
        drop(app.remove(req.world));
      } else {
        return res!(FORBIDDEN);
      }
    };

    result
      .map(|()| res!(NO_CONTENT))
      .unwrap_or_else(from_err)
  } else {
    res!(FORBIDDEN)
  }
}

pub async fn get(State(app): State<App>, Json(req): Json<GetRemoteWorldRequest>) -> Response {
  if app.server_kind().is_remote() {
    make_remote_world(&app, req.world)
      .await
      .map(|world| res!(OK, Json(world)))
      .unwrap_or_else(from_err)
  } else {
    res!(FORBIDDEN)
  }
}

pub async fn get_all(State(app): State<App>) -> Response {
  if app.server_kind().is_remote() {
    let ids = app.world_ids();
    let mut worlds = Vec::with_capacity(ids.len());

    for id in ids {
      if let Ok(world) = make_remote_world(&app, id).await {
        worlds.push(world);
      }
    }

    worlds.sort_by_key(|b| Reverse(b.config.id()));

    res!(OK, Json(worlds))
  } else {
    res!(FORBIDDEN)
  }
}

async fn make_remote_world(app: &App, id: WorldId) -> Result<RemoteWorld> {
  let database = app.database();
  let game = database.get_game(id).await?;
  let user = database
    .get_user_by_id(game.created_by)
    .await?;

  let world = app.get(id)?;
  let world = world.read().await;

  let mut active_players: usize = 0;
  let mut total_players: usize = 0;

  for player in world.players() {
    if player.is_active() {
      active_players = active_players.saturating_add(1);
    }

    total_players = total_players.saturating_add(1);
  }

  Ok(RemoteWorld {
    config: Arc::unwrap_or_clone(world.config()),
    description: game.description,
    created_by: user.player_id.into(),
    created_at: game.created_at.into(),
    updated_at: game.updated_at.into(),
    has_password: game.password.is_some(),
    current_round: world.round().id(),
    round_duration: game.round_duration.map(Into::into),
    active_players,
    total_players,
    continent_size: world.continent().size(),
  })
}

pub async fn get_limit(State(app): State<App>) -> Response {
  res!(OK, Json(app.world_limit()))
}

pub async fn get_limit_per_user(State(app): State<App>) -> Response {
  res!(OK, Json(app.world_limit_per_user()))
}
