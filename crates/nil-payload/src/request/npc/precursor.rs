// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::npc::precursor::PrecursorId;
use nil_core::world::config::WorldId;
use nil_payload_macros::FromWorld;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetPrecursorCoordsRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub id: PrecursorId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetPublicPrecursorRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  pub id: PrecursorId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize, FromWorld)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetPublicPrecursorsRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
}
