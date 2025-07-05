// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::CoreError;
use axum::response::Response;

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
  let text = err.to_string();
  match err {
    BuildingStatsNotFound(_) => res!(NOT_FOUND, text),
    BuildingStatsNotFoundForLevel(_, _) => res!(NOT_FOUND, text),
    CannotDecreaseBuildingLevel(_) => res!(BAD_REQUEST, text),
    CannotIncreaseBuildingLevel(_) => res!(BAD_REQUEST, text),
    CoordOutOfBounds(_) => res!(BAD_REQUEST, text),
    FailedToLoadWorld => res!(INTERNAL_SERVER_ERROR, text),
    FailedToSaveWorld => res!(INTERNAL_SERVER_ERROR, text),
    IndexOutOfBounds(_) => res!(BAD_REQUEST, text),
    InsufficientResources => res!(BAD_REQUEST, text),
    MineStatsNotFound(_) => res!(NOT_FOUND, text),
    MineStatsNotFoundForLevel(_, _) => res!(NOT_FOUND, text),
    NoPlayer => res!(BAD_REQUEST, text),
    NotAGuest(_) => res!(BAD_REQUEST, text),
    PlayerAlreadySpawned(_) => res!(CONFLICT, text),
    PlayerIsNotPending(_) => res!(BAD_REQUEST, text),
    PlayerNotFound(_) => res!(NOT_FOUND, text),
    RoundAlreadyStarted => res!(CONFLICT, text),
    RoundHasPendingPlayers => res!(BAD_REQUEST, text),
    RoundNotStarted => res!(BAD_REQUEST, text),
    ScriptNotFound(_) => res!(NOT_FOUND, text),
    StorageStatsNotFound(_) => res!(NOT_FOUND, text),
    StorageStatsNotFoundForLevel(_, _) => res!(NOT_FOUND, text),
    VillageNotFound(_) => res!(NOT_FOUND, text),
    WorldIsFull => res!(INTERNAL_SERVER_ERROR, text),
  }
}
