// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::CoreResult;
use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_player, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use nil_core::player::PlayerId;
use nil_core::script::{Script, ScriptId};

pub async fn add(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(scripts): Json<Vec<Script>>,
) -> Response {
  if scripts.is_empty() {
    return res!(BAD_REQUEST);
  }

  let mut world = app.world.write().await;
  let scripting = world.scripting_mut();
  let mut ids = Vec::with_capacity(scripts.len());

  for mut script in scripts {
    script.owner = player.0.clone();
    ids.push(scripting.add(script));
  }

  res!(CREATED, Json(ids))
}

pub async fn execute(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(id): Json<ScriptId>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    let scripting = world.scripting();
    let script = scripting.get(id)?;
    bail_not_player!(player.0, script.owner);
    world.execute_script(id)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn get(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(id): Json<ScriptId>,
) -> Response {
  let result: CoreResult<Script> = try {
    let world = app.world.read().await;
    let script = world.scripting().get(id)?;
    bail_not_player!(player.0, script.owner);
    script.clone()
  };

  result
    .map(|script| res!(OK, Json(script)))
    .unwrap_or_else(from_core_err)
}

pub async fn get_all(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .scripting(|s| s.get_owned_by(&id))
    .map(|scripts| res!(OK, Json(scripts)))
    .await
}

pub async fn remove(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(id): Json<ScriptId>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    let scripting = world.scripting_mut();
    let script = scripting.get(id)?;
    bail_not_player!(player.0, script.owner);
    scripting.remove(id);
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn update(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(script): Json<Script>,
) -> Response {
  if *script.id == 0 {
    res!(BAD_REQUEST, "Missing script identifier")
  } else if *player == script.owner {
    app
      .scripting_mut(|s| s.update(script))
      .map_ok(|()| res!(OK))
      .unwrap_or_else(from_core_err)
      .await
  } else {
    res!(FORBIDDEN)
  }
}
