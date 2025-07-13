// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::FutureExt;
use nil_core::player::PlayerId;
use nil_core::script::{Script, ScriptId};

pub async fn add(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(scripts): Json<Vec<Script>>,
) -> Response {
  if scripts.is_empty() {
    return res!(BAD_REQUEST);
  }

  let mut world = app.world.write().await;
  let scripting = world.scripting_mut();
  let mut ids = Vec::with_capacity(scripts.len());

  for mut script in scripts {
    script.owner = current_player.0.clone();
    ids.push(scripting.add(script));
  }

  res!(CREATED, Json(ids))
}

pub async fn get(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(id): Json<ScriptId>,
) -> Response {
  let Some(script) = app.scripting(|s| s.get(id).cloned()).await else {
    return res!(OK, Json(None::<Script>));
  };

  if *current_player == script.owner {
    res!(OK, Json(Some(script)))
  } else {
    res!(FORBIDDEN)
  }
}

pub async fn get_all(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .scripting(|s| s.get_owned_by(&id))
    .map(|scripts| res!(OK, Json(scripts)))
    .await
}

pub async fn remove(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(id): Json<ScriptId>,
) -> Response {
  let mut world = app.world.write().await;
  let scripting = world.scripting_mut();

  let mut removed = false;
  if let Some(script) = scripting.get(id) {
    if *current_player == script.owner {
      removed = scripting.remove(id);
    } else {
      return res!(FORBIDDEN);
    }
  }

  if removed { res!(OK) } else { res!(GONE) }
}

pub async fn update(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(script): Json<Script>,
) -> Response {
  if *script.id == 0 {
    res!(BAD_REQUEST, "Missing script identifier")
  } else if *current_player == script.owner {
    app
      .scripting_mut(|s| s.update(script))
      .map(|()| res!(OK))
      .await
  } else {
    res!(FORBIDDEN)
  }
}
