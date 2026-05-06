// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::continent::Coord;
use nil_core::npc::precursor::PublicPrecursor;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[cfg(feature = "axum")]
use nil_payload_macros::IntoJsonResponse;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetPrecursorCoordsResponse(pub Vec<Coord>);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetPublicPrecursorResponse(pub PublicPrecursor);

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[cfg_attr(feature = "axum", derive(IntoJsonResponse))]
#[ts(export)]
pub struct GetPublicPrecursorsResponse(pub Vec<PublicPrecursor>);
