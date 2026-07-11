// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::CoreError;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::{EitherExt, from_err};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_core::military::army::Army;
use nil_core::ruler::Ruler;
use nil_payload::request::military::*;
use nil_payload::response::military::*;
use tap::Pipe;

pub async fn cancel_maneuver(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CancelManeuverRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let mut world = world.write().await;
        if world.is_maneuver_army_owned_by(req.id, player)? {
          world.cancel_maneuver(req.id)?;
        } else {
          return res!(FORBIDDEN);
        }
      };

      result
        .map(|()| res!(OK))
        .unwrap_or_else(from_err)
    }
    Err(err) => from_err(err),
  }
}

pub async fn get_armies(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetArmiesRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      let k_size = world.continent().size();
      world
        .military()
        .indexed_armies_of(player)
        .into_iter()
        .map(|(index, army)| Ok((index.to_coord(k_size)?, army.clone())))
        .try_collect::<_, _, CoreError>()
    })
    .await
    .try_map_left(|armies| res!(OK, GetArmiesResponse(armies)))
    .into_inner()
}

pub async fn get_army(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetArmyRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let army = world
          .read()
          .await
          .military()
          .army(req.id)
          .cloned()?;

        if player != *army.owner() {
          return res!(FORBIDDEN);
        }

        army
      };

      result
        .map(|army| res!(OK, GetArmyResponse(army)))
        .unwrap_or_else(from_err)
    }
    Err(err) => from_err(err),
  }
}

pub async fn get_army_owner(
  State(app): State<App>,
  Json(req): Json<GetArmyOwnerRequest>,
) -> Response {
  app
    .military(req.world, |military| {
      military
        .army(req.id)
        .map(Army::owner)
        .cloned()
    })
    .await
    .try_map_left(|owner| res!(OK, GetArmyOwnerResponse(owner)))
    .into_inner()
}

pub async fn get_idle_armies_at(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetIdleArmiesAtRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      let player: Ruler = player.into();
      let is_own_city = world
        .continent()
        .city(req.coord)?
        .is_owned_by(player.clone());

      world
        .military()
        .idle_armies_at(req.coord)
        .filter(|army| is_own_city || army.is_owned_by(&player))
        .cloned()
        .collect_vec()
        .pipe(Ok::<_, CoreError>)
    })
    .await
    .try_map_left(|armies| res!(OK, GetIdleArmiesAtResponse(armies)))
    .into_inner()
}

pub async fn get_idle_armies_coords(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetIdleArmiesCoordsRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      let k_size = world.continent().size();
      world
        .military()
        .indexed_armies_of(player)
        .into_iter()
        .filter(|(_, army)| army.is_idle())
        .map(|(index, _)| index.to_coord(k_size))
        .try_collect()
    })
    .await
    .try_map_left(|armies| res!(OK, GetIdleArmiesCoordsResponse(armies)))
    .into_inner()
}

pub async fn get_maneuver(State(app): State<App>, Json(req): Json<GetManeuverRequest>) -> Response {
  app
    .military(req.world, |military| {
      match military.maneuver(req.id) {
        Ok(maneuver) => Ok(Some(maneuver.clone())),
        Err(CoreError::ManeuverNotFound(_)) => Ok(None),
        Err(err) => Err(err),
      }
    })
    .await
    .try_map_left(|maneuver| res!(OK, GetManeuverResponse(maneuver)))
    .into_inner()
}

pub async fn request_maneuver(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(mut req): Json<RequestManeuverRequest>,
) -> Response {
  req.request.ruler = Ruler::from(&player.0);
  app
    .world_blocking_mut(req.world, move |world| {
      if world.round().is_waiting_player(&player.0) {
        world.request_maneuver(req.request)
      } else {
        Err(CoreError::NotWaitingPlayer(player.0))
      }
    })
    .await
    .try_map_left(|id| res!(OK, RequestManeuverResponse(id)))
    .into_inner()
}
