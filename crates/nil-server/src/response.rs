// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::CoreError;
use axum::response::Response;

#[doc(hidden)]
#[macro_export]
macro_rules! res {
  ($status:ident) => {{
    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::response::Response;

    let status = StatusCode::$status;
    let body = if (status.is_client_error() || status.is_server_error())
      && let Some(reason) = status.canonical_reason()
    {
      Body::new(format!("{status} {reason}"))
    } else {
      Body::empty()
    };

    Response::builder()
      .status(status)
      .body(body)
      .unwrap()
  }};
  ($status:ident, $data:expr) => {{
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    (StatusCode::$status, $data).into_response()
  }};
}

#[expect(clippy::needless_pass_by_value)]
pub(crate) fn from_core_err(err: CoreError) -> Response {
  use CoreError::*;

  #[cfg(debug_assertions)]
  tracing::error!(message = %err, ?err);

  let text = err.to_string();
  match err {
    ArmyNotFound(..) => res!(NOT_FOUND, text),
    ArmyNotIdle(..) => res!(BAD_REQUEST, text),
    BotAlreadySpawned(..) => res!(CONFLICT, text),
    BotNotFound(..) => res!(NOT_FOUND, text),
    BuildingStatsNotFound(..) => res!(NOT_FOUND, text),
    BuildingStatsNotFoundForLevel(..) => res!(NOT_FOUND, text),
    CannotDecreaseBuildingLevel(..) => res!(BAD_REQUEST, text),
    CannotIncreaseBuildingLevel(..) => res!(BAD_REQUEST, text),
    CheatingNotAllowed => res!(BAD_REQUEST, text),
    CityNotFound(..) => res!(NOT_FOUND, text),
    FailedToReadSavedata => res!(INTERNAL_SERVER_ERROR, text),
    FailedToWriteSavedata => res!(INTERNAL_SERVER_ERROR, text),
    Forbidden => res!(FORBIDDEN, text),
    IndexOutOfBounds(..) => res!(BAD_REQUEST, text),
    InsufficientResources => res!(BAD_REQUEST, text),
    ManeuverNotFound(..) => res!(NOT_FOUND, text),
    MineStatsNotFound(..) => res!(NOT_FOUND, text),
    MineStatsNotFoundForLevel(..) => res!(NOT_FOUND, text),
    NoPlayer => res!(BAD_REQUEST, text),
    OriginIsDestination(..) => res!(BAD_REQUEST, text),
    PlayerAlreadySpawned(..) => res!(CONFLICT, text),
    PlayerIsNotPending(..) => res!(BAD_REQUEST, text),
    PlayerNotFound(..) => res!(NOT_FOUND, text),
    PrecursorNotFound(..) => res!(NOT_FOUND, text),
    RoundAlreadyStarted => res!(CONFLICT, text),
    RoundHasPendingPlayers => res!(BAD_REQUEST, text),
    RoundNotStarted => res!(BAD_REQUEST, text),
    StorageStatsNotFound(..) => res!(NOT_FOUND, text),
    StorageStatsNotFoundForLevel(..) => res!(NOT_FOUND, text),
    WallStatsNotFoundForLevel(..) => res!(NOT_FOUND, text),
    WorldIsFull => res!(INTERNAL_SERVER_ERROR, text),
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bail_not_pending {
  ($world:expr, $player:expr) => {
    if !$world.round().is_waiting_player($player) {
      use nil_core::error::Error;
      let err = Error::PlayerIsNotPending($player.clone());
      return $crate::response::from_core_err(err);
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! bail_not_player {
  ($current_player:expr, $player:expr) => {
    if $current_player != $player {
      return $crate::res!(FORBIDDEN);
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! bail_not_owned_by {
  ($world:expr, $player:expr, $coord:expr) => {
    if !$world
      .city($coord)?
      .is_owned_by_player_and(|id| $player == id)
    {
      return $crate::res!(FORBIDDEN);
    }
  };
}
