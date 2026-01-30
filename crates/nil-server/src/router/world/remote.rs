// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Result;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::world::WorldId;
use nil_database::sql_types::user::User;
use nil_payload::world::*;
use nil_server_types::RemoteWorld;
use tokio::task::spawn_blocking;

pub async fn create(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CreateRemoteWorldRequest>,
) -> Response {
  if app.server_kind().is_remote() {
    let Ok(result) = spawn_blocking(move || {
      let user = User::from(player);
      let password = req.password.as_ref();
      app
        .create_remote(&req.options)
        .user(&user)
        .maybe_world_description(req.description)
        .maybe_world_password(password)
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
    make_remote_world(&app, req.world)
      .await
      .map(|world| res!(OK, Json(world)))
      .unwrap_or_else(Response::from)
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

    worlds.sort_by_key(|b| std::cmp::Reverse(b.config.id()));

    res!(OK, Json(worlds))
  } else {
    res!(FORBIDDEN)
  }
}

async fn make_remote_world(app: &App, id: WorldId) -> Result<RemoteWorld> {
  let db = app.database();
  let world_data = db.get_world_dataless(id)?;
  let user_data = db.get_user_data_by_id(world_data.created_by)?;

  let world = app.get(id)?;
  let world = world.read().await;

  let mut active_players = 0;
  let mut total_players = 0;

  for player in world.players() {
    if player.is_active() {
      active_players += 1;
    }

    total_players += 1;
  }

  Ok(RemoteWorld {
    config: world.config().clone(),
    description: world_data.description,
    created_by: user_data.user.into(),
    created_at: world_data.created_at.into(),
    updated_at: world_data.updated_at.into(),
    has_password: world_data.password.is_some(),
    current_round: world.round().id(),
    active_players,
    total_players,
  })
}
