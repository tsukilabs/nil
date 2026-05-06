// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::npc::bot::BotId;
use nil_core::npc::precursor::PrecursorId;
use nil_core::player::PlayerId;
use nil_core::world::config::{WorldConfig, WorldId};
use nil_core::world::stats::WorldStats;
use nil_server_types::world::RemoteWorld;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct CreateRemoteWorldResponse(pub WorldId);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetRemoteWorldLimitResponse(pub u16);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetRemoteWorldLimitPerUserResponse(pub u16);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetRemoteWorldResponse(pub RemoteWorld);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetRemoteWorldsResponse(pub Vec<RemoteWorld>);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetWorldBotsResponse(pub Vec<BotId>);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetWorldConfigResponse(pub WorldConfig);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetWorldPlayersResponse(pub Vec<PlayerId>);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetWorldPrecursorsResponse(pub Vec<PrecursorId>);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetWorldStatsResponse(pub WorldStats);
