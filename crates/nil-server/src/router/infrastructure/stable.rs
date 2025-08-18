// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::CoreResult;
use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_owned_by, bail_not_pending, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::continent::Coord;
use nil_core::infrastructure::building::stable::{
  StableRecruitCatalog,
  StableRecruitOrderId,
  StableRecruitOrderRequest,
};

pub async fn add_recruit_order(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(request): Json<StableRecruitOrderRequest>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    bail_not_pending!(world, &player.0);
    bail_not_owned_by!(world, &player.0, request.coord);
    world.add_stable_recruit_order(&request)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn cancel_recruit_order(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json((coord, id)): Json<(Coord, StableRecruitOrderId)>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    bail_not_pending!(world, &player.0);
    bail_not_owned_by!(world, &player.0, coord);
    world.cancel_stable_recruit_order(coord, id)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn get_recruit_catalog(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(coord): Json<Coord>,
) -> Response {
  let result: CoreResult<StableRecruitCatalog> = try {
    let world = app.world.read().await;
    bail_not_owned_by!(world, &player.0, coord);
    let infra = world.city(coord)?.infrastructure();
    StableRecruitCatalog::new(infra)
  };

  result
    .map(|catalog| res!(OK, Json(catalog)))
    .unwrap_or_else(from_core_err)
}
