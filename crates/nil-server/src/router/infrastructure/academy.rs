// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_owned_by, bail_not_pending, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::infrastructure::building::academy::AcademyRecruitCatalog;
use nil_payload::infrastructure::academy::{
  AddAcademyRecruitOrderRequest,
  CancelAcademyRecruitOrderRequest,
  GetAcademyRecruitCatalogRequest,
};

pub async fn add_recruit_order(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<AddAcademyRecruitOrderRequest>,
) -> Response {
  let result = try {
    let mut world = app.world.write().await;
    bail_not_pending!(world, &player.0);
    bail_not_owned_by!(world, &player.0, req.request.coord);
    world.add_academy_recruit_order(&req.request)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn cancel_recruit_order(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CancelAcademyRecruitOrderRequest>,
) -> Response {
  let result = try {
    let mut world = app.world.write().await;
    bail_not_pending!(world, &player.0);
    bail_not_owned_by!(world, &player.0, req.coord);
    world.cancel_academy_recruit_order(req.coord, req.id)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn get_recruit_catalog(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetAcademyRecruitCatalogRequest>,
) -> Response {
  let result = try {
    let world = app.world.read().await;
    bail_not_owned_by!(world, &player.0, req.coord);
    let infra = world.city(req.coord)?.infrastructure();
    AcademyRecruitCatalog::new(infra)
  };

  result
    .map(|catalog| res!(OK, Json(catalog)))
    .unwrap_or_else(from_core_err)
}
