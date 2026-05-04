// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::player::{Player, PlayerId, PublicPlayer};
use nil_core::resources::maintenance::Maintenance;
use serde::{Deserialize, Serialize};

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
pub struct GetPlayerCoordsResponse(pub Vec<Coord>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
pub struct GetPlayerIdsResponse(pub Vec<PlayerId>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
pub struct GetPlayerMaintenanceResponse(pub Maintenance);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
pub struct GetPlayerResponse(pub Player);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
pub struct GetPublicPlayerResponse(pub PublicPlayer);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
pub struct GetPublicPlayersResponse(pub Vec<PublicPlayer>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
pub struct PlayerExistsResponse(pub bool);
