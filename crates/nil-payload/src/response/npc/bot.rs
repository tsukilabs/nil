// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::npc::bot::PublicBot;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetBotCoordsResponse(pub Vec<Coord>);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetPublicBotResponse(pub PublicBot);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetPublicBotsResponse(pub Vec<PublicBot>);
